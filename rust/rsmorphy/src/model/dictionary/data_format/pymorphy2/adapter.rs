use std::io;
use std::path::Path;

use string_cache::DefaultAtom;

use crate::dawg::CompletionDawg;
use crate::model::dictionary::data_format::load_helper::LoadHelper;
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

        let words = CompletionDawg::from_reader(load.reader("words.dawg")?)?;
        profiler.waypoint("words");

        let p_t_given_w = match load.reader("p_t_given_w.intdawg") {
            Ok(reader) => Some(CompletionDawg::from_reader(reader)?),
            Err(e) if e.kind() == io::ErrorKind::NotFound => None,
            Err(e) => Err(e)?,
        };
        profiler.waypoint("p_t_given_w");

        let mut prediction_suffixes_dawgs = Vec::new();
        for i in 0..paradigm_prefixes.len() {
            let dawg = CompletionDawg::from_reader(
                load.reader(format!("prediction-suffixes-{}.dawg", i))?,
            )?;
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
