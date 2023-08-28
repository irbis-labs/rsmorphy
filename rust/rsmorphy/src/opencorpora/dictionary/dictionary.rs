use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use std::path::PathBuf;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use inlinable_string::InlinableString;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use string_cache::DefaultAtom;

use crate::dawg::CompletionDawg;
use crate::dawg::Dawg;
pub use crate::dawg::HH;
pub use crate::dawg::HHH;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::dictionary::ParadigmIndex;
use crate::opencorpora::dictionary::Paradigms;
use crate::opencorpora::dictionary::*;
use crate::opencorpora::gram::Gram;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::gram::GramRow;
use crate::opencorpora::gram::GramSet;
use crate::opencorpora::kind::Case;
use crate::opencorpora::tag::TagReg;
use crate::opencorpora::util::SimpleTraceProfiler;

pub type WordsDawg = CompletionDawg<HH>;
pub type PredictionSuffixesDawg = CompletionDawg<HHH>;
pub type ConditionalProbDistDawg = CompletionDawg<HH>;

/// Open Corpora dictionary wrapper class.
#[derive(Deserialize, Serialize, Debug)]
pub struct Dictionary {
    pub grammemes: GramRegistry,
    pub gramtab: Vec<TagReg>,
    pub suffixes: Vec<DefaultAtom>,
    pub paradigms: Paradigms,
    pub words: WordsDawg,
    pub p_t_given_w: ConditionalProbDistDawg,
    pub prediction_prefixes: Dawg,
    pub prediction_suffixes_dawgs: Vec<PredictionSuffixesDawg>,
    pub paradigm_prefixes: Vec<DefaultAtom>,
    pub paradigm_prefixes_rev: Vec<(u16, DefaultAtom)>,
    pub max_suffix_length: usize,
    pub char_substitutes: HashMap<InlinableString, InlinableString>,
    pub rare_cases: [(Gram, Gram); 6],
}

impl Dictionary {
    pub fn from_bin(path: impl AsRef<Path>) -> bincode::Result<Self> {
        log::debug!("Load bin dict file {:?}", path.as_ref());
        bincode::deserialize_from(GzDecoder::new(File::open(path)?))
    }

    pub fn save_bin(&self, path: impl AsRef<Path>) -> bincode::Result<()> {
        log::debug!("Save bin dict file {:?}", path.as_ref());
        bincode::serialize_into(
            GzEncoder::new(File::create(path)?, Compression::best()),
            &self,
        )
    }

    pub fn from_file(p: impl Into<PathBuf>) -> Result<Self, LoadError> {
        let load = PathLoader::new(p);

        let mut profiler = SimpleTraceProfiler::start();

        let meta_json = {
            let meta: Vec<(String, Value)> = load.json("meta.json.gz").expect("object of `Value`s");
            HashMap::<String, Value>::from_iter(meta.into_iter())
        };
        profiler.waypoint("meta");

        let paradigm_prefixes: Vec<DefaultAtom> = {
            meta_json["compile_options"]
                .as_object()
                .unwrap()
                .get("paradigm_prefixes")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().into())
                .collect()
        };
        let max_suffix_length = {
            meta_json
                .get("prediction_options")
                .unwrap_or_else(|| &meta_json["compile_options"])
                .as_object()
                .unwrap()
                .get("max_suffix_length")
                .unwrap()
                .as_u64()
                .unwrap() as usize
        };
        profiler.waypoint("meta'");

        let paradigm_prefixes_rev = paradigm_prefixes
            .iter()
            .enumerate()
            .rev()
            .map(|(i, v)| (i as u16, v.clone()))
            .collect();
        profiler.waypoint("paradigm_prefixes_rev");

        let suffixes = load.json("suffixes.json.gz").expect("array of strings");
        profiler.waypoint("suffixes");

        let grammemes: Vec<GramRow> = load.json("grammemes.json.gz").expect("array of arrays");
        profiler.waypoint("grammemes");

        let grammemes = {
            let mut builder = GramRegistry::builder();
            for row in grammemes {
                builder.add_grammeme(row, false).unwrap();
            }
            builder.finish().unwrap()
        };
        profiler.waypoint("grammemes'");

        let gramtab: Vec<String> = load
            .json("gramtab-opencorpora-int.json.gz")
            .expect("array of strings");
        profiler.waypoint("gramtab");
        // TODO opencorpora-ext
        let gramtab = gramtab
            .into_iter()
            .map(|gram_int| TagReg::from_fmt_int(&grammemes, gram_int))
            .collect();
        profiler.waypoint("gramtab'");

        let paradigms = Paradigms::from_reader(load.reader("paradigms.array.gz")?)?;
        profiler.waypoint("paradigms");

        let words = CompletionDawg::from_reader(load.reader("words.dawg.gz")?)?;
        profiler.waypoint("words");

        let p_t_given_w = CompletionDawg::from_reader(load.reader("p_t_given_w.intdawg.gz")?)?;
        profiler.waypoint("p_t_given_w");

        let prediction_prefixes = Dawg::from_reader(load.reader("prediction-prefixes.dawg.gz")?)?;
        profiler.waypoint("prediction_prefixes");

        let prediction_suffixes_dawgs = (0..paradigm_prefixes.len())
            .map(|i| {
                CompletionDawg::from_reader(
                    load.reader(format!("prediction-suffixes-{}.dawg.gz", i))?,
                )
            })
            .collect::<Result<_, LoadError>>()?;
        profiler.waypoint("prediction_suffixes_dawgs");

        // TODO load char_substitutes
        let char_substitutes = Some(("е".into(), "ё".into())).into_iter().collect();

        let rare_cases = {
            use Case::*;

            let case_gram = |case: Case| -> Gram {
                // SAFETY: All cases should be in the registry.
                grammemes.find_gram(case.gram_int()).unwrap()
            };

            [
                (case_gram(Gen1), case_gram(Gent)),
                (case_gram(Gen2), case_gram(Gent)),
                // (case_gram(Acc1), case_gram(Accs)),
                (case_gram(Acc2), case_gram(Accs)),
                (case_gram(Loc1), case_gram(Loct)),
                (case_gram(Loc2), case_gram(Loct)),
                (case_gram(Voct), case_gram(Nomn)),
            ]
        };

        Ok(Dictionary {
            grammemes,
            gramtab,
            suffixes,
            paradigms,
            words,
            p_t_given_w,
            prediction_prefixes,
            prediction_suffixes_dawgs,
            paradigm_prefixes,
            paradigm_prefixes_rev,
            max_suffix_length,
            char_substitutes,
            rare_cases,
        })
    }

    pub fn prediction_splits(&self, word_len: usize) -> impl Iterator<Item = usize> {
        (1..=self.max_suffix_length.min(word_len)).rev()
    }

    #[inline]
    pub fn para(&self) -> &Paradigms {
        &self.paradigms
    }

    /// Return tag as a string
    #[inline]
    pub fn get_tag(&self, id: ParadigmId, idx: ParadigmIndex) -> &TagReg {
        let para = self.para().get(id).expect("paradigm");
        &self.gramtab[para[idx.index()].tag_id() as usize]
    }

    #[inline]
    pub fn iter_paradigm_info<'a: 'i, 'i>(
        &'a self,
        id: ParadigmId,
    ) -> impl Iterator<Item = (&'a str, &'a TagReg, &'a str)> + 'i {
        self.para()
            .get(id)
            .expect("paradigm")
            .iter()
            .map(move |entry: &'a ParadigmEntry| self.paradigm_entry_info(*entry))
    }

    /// Return a tuple
    ///     (prefix, tag, suffix)
    /// tuples representing the paradigm entry.
    #[inline]
    fn paradigm_entry_info(&self, entry: ParadigmEntry) -> (&str, &TagReg, &str) {
        (
            &self.paradigm_prefixes[entry.prefix_id() as usize],
            &self.gramtab[entry.tag_id() as usize],
            &self.suffixes[entry.suffix_id() as usize],
        )
    }

    /// Return word stem (given a word, paradigm and the word index).
    #[inline]
    pub(crate) fn get_stem<'a>(
        &self,
        id: ParadigmId,
        idx: ParadigmIndex,
        fixed_word: &'a str,
    ) -> &'a str {
        let paradigm = self.para().get(id).expect("paradigm");
        let para_entry = paradigm[idx.index()];
        let (prefix, _, suffix) = self.paradigm_entry_info(para_entry);
        let begin = prefix.len();
        let end = fixed_word.len() - suffix.len();
        &fixed_word[begin..end]
    }

    /// Write the normal form.
    pub fn write_normal_form(
        &self,
        mut f: impl fmt::Write,
        id: ParadigmId,
        idx: ParadigmIndex,
        fixed_word: &str,
    ) -> fmt::Result {
        if idx.is_first() {
            f.write_str(fixed_word)?;
        } else {
            let stem = self.get_stem(id, idx, fixed_word);
            let para = self.para().get(id).expect("paradigm");
            let (normal_prefix, _, normal_suffix) = self.paradigm_entry_info(para[0]);
            f.write_str(normal_prefix)?;
            f.write_str(stem)?;
            f.write_str(normal_suffix)?;
        }
        Ok(())
    }

    pub fn update_grammemes(&self, grammemes: GramSet, required: GramSet) -> GramSet {
        required
            .iter()
            .fold(grammemes | required, |grammemes, gram| {
                grammemes - self.grammemes.get_reg_by_gram(gram).incompatible
            })
    }

    pub fn fix_rare_cases(&self, mut grammemes: GramSet) -> GramSet {
        for (from, to) in self.rare_cases {
            if grammemes.contains(from) {
                grammemes.remove(from);
                grammemes.insert(to);
            }
        }
        grammemes
    }
}
