use std::io;
use std::path::Path;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use smallvec::SmallVec;

use crate::collections::dawg::DictEntry;
use crate::collections::dawg::GuideEntry;
use crate::collections::dictionary::DictionaryStorage;
use crate::collections::dictionary_meta::DictionaryMetaOwned;
use crate::collections::gram_registry::GramRegistryStorage;
use crate::collections::gram_registry::GramRow;
use crate::collections::gram_sets::GramSetsOwned;
use crate::collections::paradigm::ParadigmEntry;
use crate::collections::string_list::StringArray;
use crate::data_format::load_helper::LoadHelper;
use crate::data_format::load_helper::Reader;
use crate::data_format::pymorphy2::Pymorphy2Meta;
use crate::data_format::LoadError;
use crate::types::gram::GramSet;
use crate::util::PausedSimpleTraceProfiler;
use crate::util::SimpleTraceProfiler;

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

    pub fn load(
        self,
        initial_letters: &[char],
        char_substitutes_: &[(char, char)],
    ) -> Result<DictionaryStorage, LoadError> {
        let Self {
            load,
            meta: pm2_meta,
            profiler,
        } = self;

        let meta = DictionaryMetaOwned::new(pm2_meta.compile_options.max_suffix_length);

        let mut profiler = profiler.resume();

        let paradigm_prefixes: StringArray = {
            pm2_meta
                .compile_options
                .paradigm_prefixes
                .iter()
                .map(|v| v.as_ref().into())
                .collect()
        };

        profiler.waypoint("meta'");

        let suffixes = load.json("suffixes.json")?;
        profiler.waypoint("suffixes");

        let grammemes = {
            let grammemes: Vec<GramRow> = load.json("grammemes.json")?;
            profiler.waypoint("grammemes");

            let mut builder = GramRegistryStorage::builder();
            for row in grammemes {
                builder.add_grammeme(row, false)?;
            }
            builder.build()?
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
                .map(|gram_int| grammemes.to_ref().parse_tag(&gram_int))
                .collect()
        };
        profiler.waypoint("gramtab'");

        let paradigms = load_paradigms(load.reader("paradigms.array")?)?;
        profiler.waypoint("paradigms");

        let words = read_dawg(load.reader("words.dawg")?)?;
        profiler.waypoint("words");

        let p_t_given_w = match pm2_meta.p_t_w {
            Some(_p_t_w) => Some(read_dawg(load.reader("p_t_given_w.intdawg")?)?),
            None => None,
        };
        profiler.waypoint("p_t_given_w");

        let mut prediction_suffixes_dawgs = Vec::new();
        for i in 0..paradigm_prefixes.len() {
            let dawg = read_dawg(load.reader(format!("prediction-suffixes-{}.dawg", i))?)?;
            prediction_suffixes_dawgs.push(dawg);
        }
        profiler.waypoint("prediction_suffixes_dawgs");

        let initial_letters = initial_letters.iter().copied().collect();
        let mut char_substitutes: (SmallVec<_>, SmallVec<_>) = Default::default();
        char_substitutes.extend(char_substitutes_.iter().copied());

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
                    grammemes.to_ref().find_gram(gram_int_1)?,
                    grammemes.to_ref().find_gram(gram_int_2)?,
                ))
            })
            .collect()
        };

        let non_productive: GramSet = [
            "NUMR", "NPRO", "PRED", "PREP", "CONJ", "PRCL", "INTJ", "Apro",
        ]
        .into_iter()
        .filter_map(|gram_int| grammemes.to_ref().find_gram(gram_int))
        .collect();

        let gram_sets = GramSetsOwned::new(grammemes.to_ref(), non_productive);

        let dict = DictionaryStorage::new(
            meta,
            grammemes,
            gramtab,
            suffixes,
            paradigms,
            words,
            p_t_given_w,
            prediction_suffixes_dawgs,
            paradigm_prefixes,
            initial_letters,
            char_substitutes,
            rare_cases,
            gram_sets,
        );
        Ok(dict)
    }
}

fn read_dawg(mut reader: Reader) -> Result<(Box<[DictEntry]>, Box<[GuideEntry]>), LoadError> {
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

        units
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

        units
    };

    Ok((dict, guide))
}

fn load_paradigms(mut reader: Reader) -> io::Result<(Box<[u32]>, Box<[ParadigmEntry]>)> {
    let paradigms_count = reader.read_u16::<LittleEndian>()?;
    #[cfg(feature = "log_trace_model")]
    log::trace!("Paradigms count: {}", paradigms_count);

    let mut idx = Vec::with_capacity(paradigms_count as usize + 1);
    // Number of entries is greater than paradigms number, but we don't know how much
    // exactly. So we just reserve space for at least 1 entry per paradigm to reduce
    // reallocations.
    let mut data = Vec::with_capacity(paradigms_count as usize);
    let mut buf = Vec::with_capacity(1024);

    idx.push(0);

    for _para_num in 0..paradigms_count {
        debug_assert!(buf.is_empty(), "Buffer must be empty");

        let paradigm_len = reader.read_u16::<LittleEndian>()? as usize;
        #[cfg(feature = "log_trace_model")]
        log::trace!(
            "Paradigm number: {}, paradigm length: {}",
            _para_num + 1,
            paradigm_len
        );

        for _i in 0..paradigm_len {
            buf.push(reader.read_u16::<LittleEndian>()?)
        }

        debug_assert_eq!(buf.len(), paradigm_len, "Wrong number of entries read");
        debug_assert_eq!(
            paradigm_len % 3,
            0,
            "Paradigm length must be divisible by 3"
        );

        let paradigm_count = paradigm_len / 3;
        let (suffix, tag) = buf.split_at(paradigm_count);
        let (tag, prefix) = tag.split_at(paradigm_count);

        debug_assert_eq!(prefix.len(), paradigm_count);
        debug_assert_eq!(tag.len(), paradigm_count);
        debug_assert_eq!(suffix.len(), paradigm_count);

        let old_len = data.len();

        for i in 0..paradigm_count {
            data.push(ParadigmEntry::new(prefix[i], tag[i], suffix[i]));
        }

        debug_assert_eq!(
            data.len() - old_len,
            paradigm_count,
            "Wrong number of entries written"
        );

        idx.push(data.len() as u32);
        buf.clear();
    }
    Ok((idx.into(), data.into()))
}
