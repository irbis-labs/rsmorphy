use std::collections::HashMap;
use std::fmt;
use std::io;
use std::iter::FromIterator;
use std::path::PathBuf;

use inlinable_string::InlinableString;
use serde_json;
use serde_json::Value;
use smallvec::SmallVec;
use string_cache::DefaultAtom;

use crate::dawg::CompletionDawg;
pub use crate::dawg::HH;
pub use crate::dawg::HHH;
use crate::opencorpora::dictionary::numeral_agreement::NumeralGrammemes;
use crate::opencorpora::dictionary::value::WordValue;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::dictionary::ParadigmIndex;
use crate::opencorpora::dictionary::Paradigms;
use crate::opencorpora::dictionary::*;
use crate::opencorpora::gram::Gram;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::gram::GramRow;
use crate::opencorpora::gram::GramSet;
use crate::opencorpora::tag::TagReg;
use crate::opencorpora::util::SimpleTraceProfiler;

pub type PredictionSuffixesDawg = CompletionDawg<PredictionSuffixValue>;

/// Open Corpora dictionary wrapper class.
#[derive(Debug)]
pub struct Dictionary {
    pub grammemes: GramRegistry,
    pub gramtab: Vec<TagReg>,
    pub suffixes: Vec<DefaultAtom>,
    pub paradigms: Paradigms,
    pub words: CompletionDawg<WordValue>,
    pub p_t_given_w: Option<CompletionDawg<HH>>,
    pub prediction_suffixes_dawgs: Vec<PredictionSuffixesDawg>,
    pub paradigm_prefixes: Vec<DefaultAtom>,
    pub paradigm_prefixes_rev: Vec<(u16, DefaultAtom)>,
    pub max_suffix_length: usize,
    pub char_substitutes: HashMap<InlinableString, InlinableString>,
    pub rare_cases: SmallVec<[(Gram, Gram); 8]>,
    pub non_productive: GramSet,
    pub numeral_grammemes: NumeralGrammemes,
}

impl Dictionary {
    pub fn load_pymorphy2(p: impl Into<PathBuf>) -> Result<Self, LoadError> {
        let load = PathLoader::new(p);

        let mut profiler = SimpleTraceProfiler::start();

        let meta_json = {
            let meta: Vec<(String, Value)> = load.json("meta.json").expect("object of `Value`s");
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

        let suffixes = load.json("suffixes.json").expect("array of strings");
        profiler.waypoint("suffixes");

        let grammemes: Vec<GramRow> = load.json("grammemes.json").expect("array of arrays");
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
            .json("gramtab-opencorpora-int.json")
            .expect("array of strings");
        profiler.waypoint("gramtab");
        // TODO opencorpora-ext
        let gramtab = gramtab
            .into_iter()
            .map(|gram_int| TagReg::from_fmt_int(&grammemes, gram_int))
            .collect();
        profiler.waypoint("gramtab'");

        let paradigms = Paradigms::from_reader(load.reader("paradigms.array")?)?;
        profiler.waypoint("paradigms");

        let words = CompletionDawg::from_reader(load.reader("words.dawg")?)?;
        profiler.waypoint("words");

        let p_t_given_w = match load.reader("p_t_given_w.intdawg") {
            Ok(reader) => Some(CompletionDawg::from_reader(reader)?),
            Err(e) if e.kind() == io::ErrorKind::NotFound => None,
            Err(e) => Err(e)?,
        };
        profiler.waypoint("p_t_given_w");

        // let prediction_prefixes = Dawg::from_reader(load.reader("prediction-prefixes.dawg")?)?;
        // profiler.waypoint("prediction_prefixes");

        let prediction_suffixes_dawgs = (0..paradigm_prefixes.len())
            .map(|i| {
                CompletionDawg::from_reader(load.reader(format!("prediction-suffixes-{}.dawg", i))?)
            })
            .collect::<Result<_, LoadError>>()?;
        profiler.waypoint("prediction_suffixes_dawgs");

        // TODO load char_substitutes
        let char_substitutes = [("е".into(), "ё".into())].into_iter().collect();

        let rare_cases = {
            [
                ("gen1", "gent"),
                ("gen2", "gent"),
                ("acc1", "accs"),
                ("acc2", "accs"),
                ("loc1", "loct"),
                ("loc2", "loct"),
                ("voct", "nomn"),
            ]
            .into_iter()
            .filter_map(|(gram_int_1, gram_int_2)| {
                Some((
                    grammemes.find_gram(gram_int_1)?,
                    grammemes.find_gram(gram_int_2)?,
                ))
            })
            .collect()
        };

        let non_productive: GramSet = [
            "NUMR", "NPRO", "PRED", "PREP", "CONJ", "PRCL", "INTJ", "Apro",
        ]
        .into_iter()
        .filter_map(|gram_int| grammemes.find_gram(gram_int))
        .collect();

        let numeral_grammemes = NumeralGrammemes::new(&grammemes);

        Ok(Dictionary {
            grammemes,
            gramtab,
            suffixes,
            paradigms,
            words,
            p_t_given_w,
            // prediction_prefixes,
            prediction_suffixes_dawgs,
            paradigm_prefixes,
            paradigm_prefixes_rev,
            max_suffix_length,
            char_substitutes,
            rare_cases,
            non_productive,
            numeral_grammemes,
        })
    }

    pub fn prediction_splits(&self, word_len: usize) -> impl Iterator<Item = usize> {
        (1..=self.max_suffix_length.min(word_len - 1)).rev()
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
        for &(from, to) in &self.rare_cases {
            if grammemes.contains(from) {
                grammemes.remove(from);
                grammemes.insert(to);
            }
        }
        grammemes
    }

    #[inline(always)]
    pub fn is_productive(&self, grammemes: GramSet) -> bool {
        (grammemes & self.non_productive).is_empty()
    }

    /// Returns grammemes that are required to be present in the word.
    /// The number should be passed similar to `(number.abs() % 100) as u8`.
    pub fn numeral_agreement_grammemes(&self, grammemes: GramSet, num: u8) -> GramSet {
        enum NK {
            One,
            Little,
            Many,
        }

        let NumeralGrammemes {
            noun,
            adjf,
            prtf,
            sing,
            plur,
            nomn,
            gent,
            accs,
            femn,
            case,
        } = self.numeral_grammemes;

        let (num_10, num_100) = (num % 10, num % 100);
        let num_kind = match () {
            () if (num_10 == 1) && (num_100 != 11) => NK::One,
            () if (num_10 >= 2) && (num_10 <= 4) && (num_100 < 10 || num_100 >= 20) => NK::Little,
            () => NK::Many,
        };

        let g = grammemes;
        match num_kind {
            _ if g.is_disjoint(noun | adjf | prtf) => GramSet::empty(),

            _ if g.is_superset(noun) && g.is_disjoint(nomn | accs) => {
                (case & g) | if let NK::One = num_kind { sing } else { plur }
            }

            NK::One if g.is_superset(nomn) => sing | nomn,
            NK::One => sing | accs,

            NK::Little if g.is_superset(noun) => sing | gent,
            NK::Little if g.is_superset(femn) && g.is_intersect(adjf | prtf) => plur | nomn,

            _ => plur | gent,
        }
    }
}
