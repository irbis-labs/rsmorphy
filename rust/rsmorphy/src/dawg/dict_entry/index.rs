use std::fmt;

use crate::dawg::dict_entry::entry::PRECISION_MASK;
use crate::dawg::dict_entry::DictEntry;
use crate::dawg::dict_entry::DictLabel;
use crate::dawg::dict_entry::DictOffset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DictIndex(pub(super) u32);

impl DictIndex {
    #[inline(always)]
    pub const fn root() -> Self {
        DictIndex(0)
    }

    #[inline(always)]
    pub const fn as_entry(self) -> DictEntry {
        DictEntry(self.0)
    }

    #[inline(always)]
    pub const fn as_usize(self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    pub const fn next_index(self, offset: DictOffset, label: DictLabel) -> Self {
        DictIndex((self.0 ^ offset.0 ^ label.0) & PRECISION_MASK)
    }
}

impl fmt::LowerHex for DictIndex {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
