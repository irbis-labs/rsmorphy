pub type Paradigm<'a> = &'a [ParadigmEntry];

#[derive(Debug, Clone, Copy)]
pub struct Paradigms<'d> {
    idx: &'d [u32],
    data: &'d [ParadigmEntry],
}

impl<'d> Paradigms<'d> {
    pub(crate) fn new(idx: &'d [u32], data: &'d [ParadigmEntry]) -> Self {
        Paradigms { idx, data }
    }

    #[inline]
    pub fn get(&self, id: ParadigmId) -> Option<Paradigm<'d>> {
        let idx = id.index();
        if idx + 1 < self.idx.len() {
            let (start, end) = (self.idx[idx] as usize, self.idx[idx + 1] as usize);
            Some(&self.data[start..end])
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ParadigmEntry {
    pub(crate) prefix_id: u16,
    pub(crate) tag_id: u16,
    pub(crate) suffix_id: u16,
}

impl ParadigmEntry {
    #[inline(always)]
    pub(crate) fn new(prefix_id: u16, tag_id: u16, suffix_id: u16) -> Self {
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
