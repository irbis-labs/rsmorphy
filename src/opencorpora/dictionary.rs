use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::GzDecoder;
use serde_json;
use serde_json::Value;

use container::paradigm::{ParadigmId, ParadigmIndex};
pub use dawg::HH;
pub use dawg::HHH;
use dawg::{CompletionDawg, Dawg};
use opencorpora::grammeme::{Grammeme, GrammemeReg};
use opencorpora::paradigm::ParadigmEntry;
use opencorpora::tag::OpencorporaTagReg;

pub type WordsDawg = CompletionDawg<HH>;
pub type PredictionSuffixesDawg = CompletionDawg<HHH>;
pub type ConditionalProbDistDawg = CompletionDawg<HH>;

#[derive(Debug, Default, Clone)]
pub struct GrammemeMeta {
    pub index: usize,
    pub children: HashSet<Grammeme>,
    pub incompatible: HashSet<Grammeme>,
}

/// Open Corpora dictionary wrapper class.
#[derive(Debug, Clone)]
pub struct Dictionary {
    pub meta: HashMap<String, Value>,
    pub grammemes: HashMap<Grammeme, GrammemeReg>,
    pub grammeme_metas: HashMap<Grammeme, GrammemeMeta>,
    pub gramtab: Vec<OpencorporaTagReg>,
    pub suffixes: Vec<String>,
    pub paradigms: Vec<Vec<ParadigmEntry>>,
    pub words: WordsDawg,
    pub p_t_given_w: ConditionalProbDistDawg,
    pub prediction_prefixes: Dawg,
    pub prediction_suffixes_dawgs: Vec<PredictionSuffixesDawg>,
    pub paradigm_prefixes: Vec<String>,
    pub paradigm_prefixes_rev: Vec<(u16, String)>,
    pub prediction_splits: Vec<usize>,
    pub char_substitutes: BTreeMap<String, String>,
}

struct PathLoader {
    dict_path: PathBuf,
}

impl PathLoader {
    fn new<P>(p: P) -> Self
    where
        P: AsRef<Path>,
    {
        let dict_path = p.as_ref().into();
        PathLoader { dict_path }
    }

    fn path<S>(&self, name: S) -> PathBuf
    where
        S: AsRef<Path>,
    {
        self.dict_path.join(name)
    }

    fn reader<S>(&self, name: S) -> impl Read
    where
        S: AsRef<Path>,
    {
        GzDecoder::new(File::open(&self.path(name)).unwrap())
    }

    fn json<S, T>(&self, name: S) -> serde_json::Result<T>
    where
        S: AsRef<Path>,
        for<'de> T: ::serde::Deserialize<'de>,
    {
        serde_json::from_reader(self.reader(name))
    }
}

impl Dictionary {
    pub fn from_file<P>(p: P) -> Self
    where
        P: AsRef<Path>,
    {
        let load = PathLoader::new(p);

        let mut tm = ::std::time::Instant::now();
        let mut next_time = |l| {
            let tm2 = ::std::time::Instant::now();
            eprintln!("{} :: {:?}", l, (tm2 - tm));
            tm = tm2;
        };

        let meta: Vec<(String, Value)> = load.json("meta.json.gz").expect("object of `Value`s");
        let meta = HashMap::from_iter(meta.into_iter());
        next_time("meta");

        let paradigm_prefixes: Vec<String> = {
            meta["compile_options"]
                .as_object()
                .unwrap()
                .get("paradigm_prefixes")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect()
        };
        let max_suffix_length = {
            meta.get("prediction_options")
                .unwrap_or_else(|| &meta["compile_options"])
                .as_object()
                .unwrap()
                .get("max_suffix_length")
                .unwrap()
                .as_u64()
                .unwrap() as usize
        };
        let prediction_splits = (1..=max_suffix_length).rev().collect();
        next_time("meta'");

        let paradigm_prefixes_rev = paradigm_prefixes
            .iter()
            .enumerate()
            .rev()
            .map(|(i, v)| (i as u16, v.clone()))
            .collect();
        next_time("paradigm_prefixes_rev");

        let suffixes = load.json("suffixes.json.gz").expect("array of strings");
        next_time("suffixes");

        let gramtab: Vec<String> = load
            .json("gramtab-opencorpora-int.json.gz")
            .expect("array of strings");
        next_time("gramtab");
        // TODO opencorpora-ext
        let gramtab = gramtab.into_iter().map(OpencorporaTagReg::new).collect();
        next_time("gramtab'");

        // TODO join `grammemes` and `grammeme_metas` into one set
        let grammemes: Vec<Vec<Value>> = load.json("grammemes.json.gz").expect("array of `Value`s");
        next_time("grammemes");
        let grammemes = grammemes
            .into_iter()
            .map(GrammemeReg::from_json)
            .map(|gr| (gr.name.clone(), gr));
        let grammemes = HashMap::from_iter(grammemes);
        let grammeme_metas = {
            let mut grammeme_metas = HashMap::<Grammeme, GrammemeMeta>::default();
            for (index, grammeme) in grammemes.keys().enumerate() {
                if !grammeme_metas.contains_key(grammeme) {
                    grammeme_metas.insert(
                        grammeme.clone(),
                        GrammemeMeta {
                            index,
                            ..GrammemeMeta::default()
                        },
                    );
                }
            }
            for (grammeme, gram_reg) in &grammemes {
                if let Some(ref parent) = gram_reg.parent {
                    grammeme_metas
                        .get_mut(parent)
                        .unwrap()
                        .children
                        .insert(grammeme.clone());
                }
            }

            // Extra incompatible:   'plur': set(['GNdr'])
            // {u'plur': set([u'GNdr', u'masc', u'femn', u'neut'])}
            let plur = Grammeme::new("plur");
            let gndr = Grammeme::new("GNdr");
            let mut extra_incompatible = hashset!{ gndr.clone() };
            extra_incompatible.extend(grammeme_metas[&gndr].children.iter().cloned());

            for grammeme in grammemes.keys() {
                let gm: &mut GrammemeMeta = grammeme_metas.get_mut(grammeme).unwrap();
                if grammeme == &plur {
                    gm.incompatible
                        .extend(extra_incompatible.iter().cloned().filter(|v| v != grammeme));
                }
                gm.incompatible
                    .extend(gm.children.iter().cloned().filter(|v| v != grammeme));
            }
            grammeme_metas
        };
        next_time("grammemes'");

        let paradigms = load_paradigms(&mut load.reader("paradigms.array.gz"));
        next_time("paradigms");
        let words = CompletionDawg::from_reader(&mut load.reader("words.dawg.gz"));
        next_time("words");
        let p_t_given_w = CompletionDawg::from_reader(&mut load.reader("p_t_given_w.intdawg.gz"));
        next_time("p_t_given_w");
        let prediction_prefixes =
            Dawg::from_reader(&mut load.reader("prediction-prefixes.dawg.gz"));
        next_time("prediction_prefixes");
        let prediction_suffixes_dawgs = Vec::from_iter((0..paradigm_prefixes.len()).map(|i| {
            CompletionDawg::from_reader(
                &mut load.reader(format!("prediction-suffixes-{}.dawg.gz", i)),
            )
        }));
        next_time("prediction_suffixes_dawgs");

        // TODO load char_substitutes
        let char_substitutes = btreemap!{"е".into() => "ё".into()};

        Dictionary {
            meta,
            grammemes,
            grammeme_metas,
            gramtab,
            suffixes,
            paradigms,
            words,
            p_t_given_w,
            prediction_prefixes,
            prediction_suffixes_dawgs,
            paradigm_prefixes,
            paradigm_prefixes_rev,
            prediction_splits,
            char_substitutes,
        }
    }

    pub fn get_paradigm<Id>(&self, id: Id) -> &[ParadigmEntry]
    where
        Id: Into<ParadigmId>,
    {
        let id = id.into();
        &self.paradigms[id.value() as usize]
    }

    pub fn get_paradigm_entry<Id, Idx>(&self, id: Id, idx: Idx) -> ParadigmEntry
    where
        Id: Into<ParadigmId>,
        Idx: Into<ParadigmIndex>,
    {
        let idx = idx.into();
        self.get_paradigm(id)[idx.value() as usize]
    }

    /// Return tag as a string
    pub fn get_tag(&self, id: ParadigmId, idx: ParadigmIndex) -> &OpencorporaTagReg {
        &self.gramtab[self.get_paradigm_entry(id, idx).tag_id as usize]
    }

    /// Return a list of
    ///     (prefix, tag, suffix)
    /// tuples representing the paradigm.
    pub fn build_paradigm_info(&self, id: ParadigmId) -> Vec<(&str, &OpencorporaTagReg, &str)> {
        Vec::from_iter(self.iter_paradigm_info(id))
    }

    pub fn iter_paradigm_info<'a: 'i, 'i>(
        &'a self,
        id: ParadigmId,
    ) -> impl Iterator<Item = (&'a str, &'a OpencorporaTagReg, &'a str)> + 'i {
        self.paradigms[id.value() as usize]
            .iter()
            .map(move |entry: &'a ParadigmEntry| self.paradigm_entry_info(*entry))
    }

    /// Return a tuple
    ///     (prefix, tag, suffix)
    /// tuples representing the paradigm entry.
    pub fn paradigm_entry_info(&self, entry: ParadigmEntry) -> (&str, &OpencorporaTagReg, &str) {
        (
            &self.paradigm_prefixes[entry.prefix_id as usize],
            &self.gramtab[entry.tag_id as usize],
            &self.suffixes[entry.suffix_id as usize],
        )
    }

    /// Return word stem (given a word, paradigm and the word index).
    pub fn get_stem<'a>(&self, id: ParadigmId, idx: ParadigmIndex, fixed_word: &'a str) -> &'a str {
        let (prefix, _, suffix) = self.paradigm_entry_info(self.get_paradigm_entry(id, idx));
        &fixed_word[prefix.len()..fixed_word.len() - suffix.len()]
    }

    /// Build a normal form.
    pub fn build_normal_form<'a>(
        &self,
        id: ParadigmId,
        idx: ParadigmIndex,
        fixed_word: &'a str,
    ) -> Cow<'a, str> {
        if idx.is_first() {
            fixed_word.into()
        } else {
            let stem = self.get_stem(id, idx, fixed_word);
            let (normal_prefix, _, normal_suffix) =
                self.paradigm_entry_info(self.get_paradigm(id)[0]);
            format!("{}{}{}", normal_prefix, stem, normal_suffix).into()
        }
    }

    /// Write a normal form.
    pub fn write_normal_form<'a, W: fmt::Write>(
        &self,
        f: &mut W,
        id: ParadigmId,
        idx: ParadigmIndex,
        fixed_word: &'a str,
    ) -> fmt::Result {
        if idx.is_first() {
            write!(f, "{}", fixed_word)
        } else {
            let stem = self.get_stem(id, idx, fixed_word);
            let (normal_prefix, _, normal_suffix) =
                self.paradigm_entry_info(self.get_paradigm(id)[0]);
            write!(f, "{}{}{}", normal_prefix, stem, normal_suffix)
        }
    }
}

fn load_paradigms<R: Read>(reader: &mut R) -> Vec<Vec<ParadigmEntry>> {
    let paradigms_count = reader.read_u16::<LittleEndian>().unwrap();
    (0..paradigms_count)
        .map(|_| {
            let paradigm_len = reader.read_u16::<LittleEndian>().unwrap();
            (0..paradigm_len)
                .map(|_| reader.read_u16::<LittleEndian>().unwrap())
                .collect::<Vec<u16>>()
        })
        .map(ParadigmEntry::build)
        .collect()
}
