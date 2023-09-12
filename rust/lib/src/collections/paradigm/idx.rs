#[derive(Clone, Copy, Debug, Default, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ParadigmIndex(u16);

impl ParadigmIndex {
    #[inline(always)]
    pub(crate) fn new(index: u16) -> Self {
        ParadigmIndex(index)
    }

    pub unsafe fn new_unchecked(index: u16) -> Self {
        Self::new(index)
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

    #[inline(always)]
    pub fn is_first(self) -> bool {
        self.0 == 0
    }
}
