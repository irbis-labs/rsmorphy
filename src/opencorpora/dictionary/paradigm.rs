use std::io::Read;

use serde::{Deserialize, Serialize};
use byteorder::{LittleEndian, ReadBytesExt};

pub type Paradigm<'a> = &'a [ParadigmEntry];

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct Paradigms {
    idx: Vec<u32>,
    data: Vec<ParadigmEntry>,
}

impl Paradigms {
    pub fn from_reader<R: Read>(reader: &mut R) -> Self {
        let paradigms_count = reader.read_u16::<LittleEndian>().unwrap();
        let mut idx = Vec::with_capacity(paradigms_count as usize + 1);
        let mut data = Vec::new();
        idx.push(0);
        for _ in 0..paradigms_count {
            let paradigm_len = reader.read_u16::<LittleEndian>().unwrap();
            let paradigm = (0..paradigm_len)
                .map(|_| reader.read_u16::<LittleEndian>().unwrap())
                .collect::<Vec<u16>>();
            let paradigm = ParadigmEntry::build(paradigm);
            idx.push(idx.last().unwrap() + paradigm.len() as u32);
            data.extend(paradigm.into_iter())
        }
        Paradigms { idx, data }
    }

    #[inline]
    pub fn get<Id>(&self, id: Id) -> Option<Paradigm>
    where
        Id: Into<ParadigmId>,
    {
        let id = id.into().index();
        if id + 1 < self.idx.len() {
            let (start, end) = (self.idx[id] as usize, self.idx[id + 1] as usize);
            Some(&self.data[start..end])
        } else {
            None
        }
    }
}

use std::iter::FromIterator;

#[derive(Deserialize, Serialize)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ParadigmEntry {
    pub prefix_id: u16,
    pub tag_id: u16,
    pub suffix_id: u16,
}

impl ParadigmEntry {
    pub fn build<P: AsRef<[u16]>>(paradigm: P) -> Vec<Self> {
        let paradigm = paradigm.as_ref();
        assert_eq!(0, paradigm.len() % 3, "Wrong paradigm length");
        let paradigm_len = paradigm.len() / 3;
        Vec::from_iter((0..paradigm_len).map(|idx| ParadigmEntry {
            suffix_id: paradigm[idx],
            tag_id: paradigm[paradigm_len + idx],
            prefix_id: paradigm[paradigm_len * 2 + idx],
        }))
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ParadigmId(u16);

#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ParadigmIndex(u16);

impl ParadigmId {
    pub fn new<I>(id: I) -> Self
    where
        I: Into<u16>,
    {
        ParadigmId(id.into())
    }

    pub fn new_checked_u<I>(id: I) -> Self
    where
        I: Into<u128>,
    {
        let id = id.into();
        assert!(id <= ::std::u16::MAX.into());
        Self::new(id as u16)
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn value(self) -> u16 {
        self.0
    }
}

impl From<u8> for ParadigmId {
    fn from(id: u8) -> Self {
        Self::new(id)
    }
}

impl From<u16> for ParadigmId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<u32> for ParadigmId {
    fn from(index: u32) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<u64> for ParadigmId {
    fn from(index: u64) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<u128> for ParadigmId {
    fn from(index: u128) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<usize> for ParadigmId {
    fn from(index: usize) -> Self {
        Self::new_checked_u(index as u128)
    }
}

impl ParadigmIndex {
    pub fn new<I>(index: I) -> Self
    where
        I: Into<u16>,
    {
        ParadigmIndex(index.into())
    }

    pub fn new_checked_u<I>(id: I) -> Self
    where
        I: Into<u128>,
    {
        let id = id.into();
        assert!(id <= ::std::u16::MAX.into());
        Self::new(id as u16)
    }


    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn value(self) -> u16 {
        self.0
    }

    #[inline]
    pub fn is_first(self) -> bool {
        self.0 == 0
    }
}

impl From<u8> for ParadigmIndex {
    fn from(id: u8) -> Self {
        Self::new(id)
    }
}

impl From<u16> for ParadigmIndex {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<u32> for ParadigmIndex {
    fn from(index: u32) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<u64> for ParadigmIndex {
    fn from(index: u64) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<u128> for ParadigmIndex {
    fn from(index: u128) -> Self {
        Self::new_checked_u(index)
    }
}

impl From<usize> for ParadigmIndex {
    fn from(index: usize) -> Self {
        Self::new_checked_u(index as u128)
    }
}
