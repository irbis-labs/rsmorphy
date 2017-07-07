use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::hash_map::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::Read;
//use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;

use flate2::read::GzDecoder;
use serde_json;
use serde_json::Value;

use ::dawg::CompletionDawg;
use ::dawg::Dawg;
pub use ::dawg::HH;
pub use ::dawg::HHH;
use ::opencorpora::tag::OpencorporaTagReg;
use ::opencorpora::grammeme::Grammeme;
use ::opencorpora::grammeme::GrammemeReg;
use ::opencorpora::paradigm::ParadigmEntry;
use ::util::u16_from_slice;


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


impl Dictionary {
    pub fn from_file<P>(p: P) -> Self where P: AsRef<Path> {
        let p = p.as_ref();
        let meta = meta_from_json(load_json(&p.join("meta.json.gz")));
        let paradigm_prefixes: Vec<String> = {
            meta["compile_options"]
                .as_object().unwrap().get("paradigm_prefixes").unwrap()
                .as_array().unwrap()
                .iter().map(|v| v.as_str().unwrap().to_owned() )
                .collect()
        };
        let max_suffix_length: usize = {
            meta.get("prediction_options")
                .unwrap_or_else(|| &meta["compile_options"])
                .as_object().unwrap().get("max_suffix_length").unwrap()
                .as_u64().unwrap() as usize
        };

        // TODO join `grammemes` and `grammeme_metas` into one set
        let grammemes = GrammemeReg::map_from_json(load_json(&p.join("grammemes.json.gz")));
        let grammeme_metas = {
            let mut grammeme_metas: HashMap<Grammeme, GrammemeMeta> = Default::default();
            for (index, grammeme) in grammemes.keys().enumerate() {
                grammeme_metas
                    .entry(grammeme.clone())
                    .or_insert_with(|| GrammemeMeta {
                        index: index,
                        children: Default::default(),
                        incompatible: Default::default(),
                    });
            }
            for (grammeme, gram_reg) in &grammemes {
                if let Some(ref parent) = gram_reg.parent {
                    grammeme_metas
                        .get_mut(parent).unwrap()
                        .children.insert(grammeme.clone());
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
                    gm.incompatible.extend(
                        extra_incompatible.iter().cloned().filter(|v| v != grammeme));
                }
                gm.incompatible.extend(gm.children.iter().cloned().filter(|v| v != grammeme));
            }
            grammeme_metas
        };


        Dictionary {
            meta: meta,
            grammemes: grammemes,
            grammeme_metas: grammeme_metas,
            gramtab: OpencorporaTagReg::vec_from_json(load_json(
                &p.join("gramtab-opencorpora-int.json.gz") // TODO opencorpora-ext
            )),
            suffixes: suffixes_from_json(load_json(&p.join("suffixes.json.gz"))),
            paradigms: load_paradigms(&p.join("paradigms.array.gz")),
            words: CompletionDawg::from_file(&p.join("words.dawg.gz")),
            p_t_given_w: CompletionDawg::from_file(&p.join("p_t_given_w.intdawg.gz")),
            prediction_prefixes: Dawg::from_file(&p.join("prediction-prefixes.dawg.gz")),
            prediction_suffixes_dawgs: Vec::from_iter(
                (0..paradigm_prefixes.len()).into_iter().map(
                    |i| CompletionDawg::from_file(&p.join(format!("prediction-suffixes-{}.dawg.gz", i)))
                )
            ),
            paradigm_prefixes: paradigm_prefixes.clone(),
            paradigm_prefixes_rev: paradigm_prefixes.into_iter()
                .enumerate().map(|(i, v)| (i as u16, v)).rev().collect(),
            prediction_splits: (1 .. 1 + max_suffix_length).rev().collect(),
            // TODO load char_substitutes
            char_substitutes: btreemap!{"е".into() => "ё".into()}
            // "\u{0435}" => "\u{0451}"
            // vec![0xd0, 0xb5] => vec![0xd1, 0x91]
        }
    }

    /// Return tag as a string
    pub fn build_tag_info(&self, para_id: u16, idx: u16) -> &OpencorporaTagReg {
        &self.gramtab[self.paradigms[para_id as usize][idx as usize].tag_id as usize]
    }

    /// Return a list of
    ///     (prefix, tag, suffix)
    /// tuples representing the paradigm.
    pub fn build_paradigm_info(&self, para_id: u16) -> Vec<(&str, &OpencorporaTagReg, &str)> {
        Vec::from_iter(self.iter_paradigm_info(para_id))
    }

    pub fn iter_paradigm_info<'a: 'i, 'i>(&'a self, para_id: u16)
        -> impl Iterator<Item = (&'a str, &'a OpencorporaTagReg, &'a str)> + 'i
    {
        self.paradigms[para_id as usize].iter().map(
            move |entry: &'a ParadigmEntry| { self.paradigm_entry_info(*entry) }
        )
    }

    /// Return a tuple
    ///     (prefix, tag, suffix)
    /// tuples representing the paradigm entry.
    pub fn paradigm_entry_info(&self, entry: ParadigmEntry) -> (&str, &OpencorporaTagReg, &str) {
        (
            self.paradigm_prefixes[entry.prefix_id as usize].as_str(),
            &self.gramtab[entry.tag_id as usize],
            self.suffixes[entry.suffix_id as usize].as_str()
        )
    }

    /// Return word stem (given a word, paradigm and the word index).
    pub fn build_stem<'a>(&self, para_id: u16, idx: u16, fixed_word: &'a str) -> &'a str {
        let (prefix, _, suffix) = self.paradigm_entry_info(self.paradigms[para_id as usize][idx as usize]);
        &fixed_word[prefix.len() .. fixed_word.len() - suffix.len()]
    }

    /// Build a normal form.
    pub fn build_normal_form<'a>(&self, para_id: u16, idx: u16, fixed_word: &'a str) -> Cow<'a, str> {
        if idx == 0 { return Cow::Borrowed(fixed_word); }
        let stem = self.build_stem(para_id, idx, fixed_word);
        let (normal_prefix, _, normal_suffix) = self.paradigm_entry_info(self.paradigms[para_id as usize][0]);
        Cow::Owned(format!("{}{}{}", normal_prefix, stem, normal_suffix))
    }

    /// Write a normal form.
    pub fn write_normal_form<'a, W: fmt::Write>(&self, f: &mut W, para_id: u16, idx: u16, fixed_word: &'a str) -> fmt::Result {
        if idx == 0 {
            write!(f, "{}", fixed_word)
        } else {
            let stem = self.build_stem(para_id, idx, fixed_word);
            let (normal_prefix, _, normal_suffix) = self.paradigm_entry_info(self.paradigms[para_id as usize][0]);
            write!(f, "{}{}{}", normal_prefix, stem, normal_suffix)
        }
    }
}

fn load_json(p: &Path) -> Value {
    serde_json::from_reader(GzDecoder::new(File::open(p).unwrap()).unwrap()).unwrap()
}


pub fn meta_from_json(data: Value) -> HashMap<String, Value> {
    let data = match data {
        Value::Array(data) => data,
        _ => unreachable!(),
    };
    data.into_iter().map(|tuple| {
        //trace!("{:?}", tuple);
        let tuple = tuple.as_array().unwrap();
        let name = tuple[0].as_str().unwrap();
        (name.to_owned(), tuple[1].clone())
    })
        .collect()
}


fn suffixes_from_json(data: Value) -> Vec<String> {
    let data = match data {
        Value::Array(data) => data,
        _ => unreachable!(),
    };
    data.into_iter().map(|v| {
        let v = v.as_str().unwrap();
        v.to_owned()
    })
        .collect()
}


fn load_paradigms(p: &Path) -> Vec<Vec<ParadigmEntry>> {
    let mut f = &mut GzDecoder::new(File::open(p).unwrap()).unwrap();
    let mut buf16 = [0u8; 2];

    f.read_exact(&mut buf16).unwrap();
    let paradigms_count = u16::from_le(u16_from_slice(&buf16)) as usize;
    let mut paradigms: Vec<Vec<ParadigmEntry>> = Vec::with_capacity(paradigms_count);
    paradigms.extend((0 .. paradigms_count).map(
        |_i| {
            f.read_exact(&mut buf16).unwrap();
            let paradigm_len = u16::from_le(u16_from_slice(&buf16)) as usize;

            let mut paradigm: Vec<u16> = Vec::with_capacity(paradigm_len);
            paradigm.extend((0 .. paradigm_len).map(
                |_ii| {
                    f.read_exact(&mut buf16).unwrap();
                    u16::from_le(u16_from_slice(&buf16))
                }
            ));
            ParadigmEntry::build(paradigm)
        }
    ));
    paradigms
}
