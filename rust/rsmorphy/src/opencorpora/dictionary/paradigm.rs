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
    pub fn from_reader<R: Read>(reader: &mut R) -> Self {
        let paradigms_count = reader.read_u16::<LittleEndian>().unwrap();

        let mut idx = Vec::with_capacity(paradigms_count as usize + 1);
        // Number of entries is greater than paradigms number, but we don't know how much
        // exactly. So we just reserve space for at least 1 entry per paradigm to reduce
        // reallocations.
        let mut data = Vec::with_capacity(paradigms_count as usize);
        let mut buf = Vec::new();

        idx.push(0);

        for _ in 0..paradigms_count {
            let paradigm_len = reader.read_u16::<LittleEndian>().unwrap();
            for _i in 0..paradigm_len {
                buf.push(reader.read_u16::<LittleEndian>().unwrap())
            }

            assert_eq!(buf.len() % 3, 0, "Wrong paradigm length");
            let paradigm_len = buf.len() / 3;
            let (prefix, tag) = buf.split_at(paradigm_len);
            let (tag, suffix) = tag.split_at(paradigm_len);

            #[inline(always)]
            fn iter(arr: &[u16]) -> impl Iterator<Item = u16> + '_ {
                arr.iter().copied()
            }
            data.extend(iter(prefix).zip(iter(tag)).zip(iter(suffix)).map(
                |((prefix_id, tag_id), suffix_id)| ParadigmEntry::new(prefix_id, tag_id, suffix_id),
            ));

            idx.push(data.len() as u32);
            buf.clear();
        }
        Paradigms { idx, data }
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
