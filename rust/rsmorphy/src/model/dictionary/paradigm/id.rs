#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ParadigmId(u16);

impl ParadigmId {
    #[inline(always)]
    pub fn new(id: u16) -> Self {
        ParadigmId(id)
    }

    pub unsafe fn new_unchecked(id: u16) -> Self {
        Self::new(id)
    }

    pub fn into_inner(self) -> u16 {
        self.0
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
