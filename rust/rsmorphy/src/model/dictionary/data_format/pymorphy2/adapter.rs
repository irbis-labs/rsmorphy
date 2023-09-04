use std::io;
use std::path::Path;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use string_cache::DefaultAtom;

use crate::dawg::CompletionDawg;
use crate::dawg::DawgDictionary;
use crate::dawg::DawgGuide;
use crate::dawg::DawgValue;
use crate::dawg::DictEntry;
use crate::dawg::GuideEntry;
use crate::model::dictionary::data_format::load_helper::LoadHelper;
use crate::model::dictionary::data_format::load_helper::Reader;
use crate::model::dictionary::data_format::pymorphy2::Pymorphy2Meta;
use crate::model::dictionary::numeral_agreement::NumeralGrammemes;
use crate::model::dictionary::LoadError;
use crate::model::gram::GramRegistry;
use crate::model::gram::GramRow;
use crate::model::tag::TagReg;
use crate::model::util::PausedSimpleTraceProfiler;
use crate::model::util::SimpleTraceProfiler;
use crate::model::Dictionary;
use crate::model::Paradigms;
use crate::GramSet;

/// Pymorphy2 data format adapter.
#[derive(Debug, Clone)]
pub struct Pymorphy2Adapter {
    load: LoadHelper,
    meta: Pymorphy2Meta,
    profiler: PausedSimpleTraceProfiler,
}

impl Pymorphy2Adapter {
    pub fn open(dir: impl AsRef<Path>) -> Result<Self, LoadError> {
        let load = LoadHelper::new(dir)?;

        let mut profiler = SimpleTraceProfiler::start();

        let meta = load.json("meta.json")?;
        profiler.waypoint("meta");

        let profiler = profiler.pause();

        Ok(Pymorphy2Adapter {
            load,
            meta,
            profiler,
        })
    }

    pub fn meta(&self) -> &Pymorphy2Meta {
        &self.meta
    }

    pub fn load(self) -> Result<Dictionary, LoadError> {
        let Self {
            load,
            meta,
            profiler,
        } = self;

        let mut profiler = profiler.resume();

        let paradigm_prefixes: Vec<DefaultAtom> = {
            meta.compile_options
                .paradigm_prefixes
                .iter()
                .map(|v| v.as_ref().into())
                .collect()
        };

        // TODO remove, we can iter over paradigm_prefixes directly
        let paradigm_prefixes_rev = paradigm_prefixes
            .iter()
            .cloned()
            .enumerate()
            .rev()
            .map(|(i, v)| (i as u16, v))
            .collect();

        let max_suffix_length = meta.compile_options.max_suffix_length as usize;

        profiler.waypoint("meta'");

        let suffixes = load.json("suffixes.json")?;
        profiler.waypoint("suffixes");

        let grammemes = {
            let grammemes: Vec<GramRow> = load.json("grammemes.json")?;
            profiler.waypoint("grammemes");

            let mut builder = GramRegistry::builder();
            for row in grammemes {
                builder.add_grammeme(row, false)?;
            }
            builder.finish()?
        };
        profiler.waypoint("grammemes'");

        // TBD Do we need opencorpora-ext?
        let gramtab = {
            let gramtab: Vec<String> = load
                .json("gramtab-opencorpora-int.json")
                .expect("array of strings");
            profiler.waypoint("gramtab");

            gramtab
                .into_iter()
                .map(|gram_int| TagReg::from_fmt_int(&grammemes, gram_int))
                .collect()
        };
        profiler.waypoint("gramtab'");

        let paradigms = Paradigms::from_reader(load.reader("paradigms.array")?)?;
        profiler.waypoint("paradigms");

        let words = read_completion_dawg(load.reader("words.dawg")?)?;
        profiler.waypoint("words");

        let p_t_given_w = match meta.p_t_w {
            Some(_p_t_w) => Some(read_completion_dawg(load.reader("p_t_given_w.intdawg")?)?),
            None => None,
        };
        profiler.waypoint("p_t_given_w");

        let mut prediction_suffixes_dawgs = Vec::new();
        for i in 0..paradigm_prefixes.len() {
            let dawg =
                read_completion_dawg(load.reader(format!("prediction-suffixes-{}.dawg", i))?)?;
            prediction_suffixes_dawgs.push(dawg);
        }
        profiler.waypoint("prediction_suffixes_dawgs");

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
}

fn read_completion_dawg<V: DawgValue>(reader: Reader) -> Result<CompletionDawg<V>, LoadError> {
    let (dict, guide) = read_dawg(reader)?;
    Ok(CompletionDawg::new(dict, guide))
}

fn read_dawg(mut reader: Reader) -> Result<(DawgDictionary, DawgGuide), LoadError> {
    let dict = {
        let units_count = reader.read_u32::<LittleEndian>()?;
        if units_count == 0 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Dictionary is empty",
            ))?;
        }

        let units = (0..units_count)
            .map(|_| {
                let unit = reader.read_u32::<LittleEndian>()?;
                Ok(unsafe { DictEntry::from_u32_unchecked(unit) })
            })
            .collect::<io::Result<_>>()?;

        DawgDictionary::new(units)
    };

    let guide = {
        let units_count = reader.read_u32::<LittleEndian>()?;
        // if units_count == 0 {
        //     return Err(io::Error::new(io::ErrorKind::InvalidData, "Guide is empty"));
        // }

        let units = (0..units_count)
            .map(|_| {
                let child = reader.read_u8()?;
                let sibling = reader.read_u8()?;
                Ok(GuideEntry::new(child, sibling))
            })
            .collect::<io::Result<_>>()?;

        DawgGuide::new(units)
    };

    Ok((dict, guide))
}
