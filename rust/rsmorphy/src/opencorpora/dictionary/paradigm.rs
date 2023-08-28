use std::io;
use std::io::Read;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use serde::Deserialize;
use serde::Serialize;

pub type Paradigm<'a> = &'a [ParadigmEntry];

#[derive(Deserialize, Serialize, Debug)]
pub struct Paradigms {
    idx: Vec<u32>,
    data: Vec<ParadigmEntry>,
}

impl Paradigms {
    pub fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let paradigms_count = reader.read_u16::<LittleEndian>()?;
        #[cfg(feature = "trace_opencorpora")]
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
            #[cfg(feature = "trace_opencorpora")]
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

            #[inline(always)]
            fn iter(arr: &[u16]) -> impl Iterator<Item = u16> + '_ {
                arr.iter().copied()
            }
            data.extend(iter(prefix).zip(iter(tag)).zip(iter(suffix)).map(
                |((prefix_id, tag_id), suffix_id)| ParadigmEntry::new(prefix_id, tag_id, suffix_id),
            ));

            debug_assert_eq!(
                data.len() - old_len,
                paradigm_count,
                "Wrong number of entries written"
            );

            idx.push(data.len() as u32);
            buf.clear();
        }
        Ok(Paradigms { idx, data })
    }

    #[inline]
    pub fn get(&self, id: ParadigmId) -> Option<Paradigm> {
        let idx = id.index();
        if idx + 1 < self.idx.len() {
            let (start, end) = (self.idx[idx] as usize, self.idx[idx + 1] as usize);
            Some(&self.data[start..end])
        } else {
            None
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
pub struct ParadigmEntry {
    pub(crate) prefix_id: u16,
    pub(crate) tag_id: u16,
    pub(crate) suffix_id: u16,
}

impl ParadigmEntry {
    #[inline(always)]
    fn new(prefix_id: u16, tag_id: u16, suffix_id: u16) -> Self {
        ParadigmEntry {
            prefix_id,
            tag_id,
            suffix_id,
        }
    }

    #[inline(always)]
    pub fn prefix_id(&self) -> u16 {
        self.prefix_id
    }

    #[inline(always)]
    pub fn tag_id(&self) -> u16 {
        self.tag_id
    }

    #[inline(always)]
    pub fn suffix_id(&self) -> u16 {
        self.suffix_id
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ParadigmId(u16);

#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ParadigmIndex(u16);

impl ParadigmId {
    #[inline(always)]
    pub fn new(id: u16) -> Self {
        ParadigmId(id.into())
    }

    #[inline(always)]
    pub fn index(self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    pub fn value(self) -> u16 {
        self.0
    }
}

impl ParadigmIndex {
    #[inline(always)]
    pub(crate) fn new(index: u16) -> Self {
        ParadigmIndex(index.into())
    }

    #[inline(always)]
    pub fn index(self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    pub fn value(self) -> u16 {
        self.0
    }

    #[inline(always)]
    pub fn is_first(self) -> bool {
        self.0 == 0
    }
}
