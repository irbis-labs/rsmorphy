use std::fmt;

use crate::dawg::dict_entry::value::DictValue;
use crate::dawg::dict_entry::DictLabel;
use crate::dawg::dict_entry::DictOffset;

pub(super) const PRECISION_MASK: u32 = 0xFFFF_FFFF;

#[allow(dead_code)]
pub(super) const OFFSET_MAX: u32 = 0x0020_0000; // 1 << 21
pub(super) const IS_LEAF_BIT: u32 = 0x8000_0000; // 1 << 31
pub(super) const HAS_LEAF_BIT: u32 = 0x0000_0100; // 1 << 8
pub(super) const EXTENSION_BIT: u32 = 0x0000_0200; // 1 << 9

pub(super) const VALUE_BITS: u32 = IS_LEAF_BIT ^ PRECISION_MASK;
pub(super) const LABEL_BITS: u32 = IS_LEAF_BIT | 0x0000_00FF;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DictEntry(pub(super) u32);

impl DictEntry {
    #[inline(always)]
    pub const unsafe fn from_u32_unchecked(unit: u32) -> Self {
        DictEntry(unit)
    }

    /// Check if a unit has a leaf as a child or not.
    #[inline(always)]
    pub const fn has_leaf(&self) -> bool {
        self.0 & HAS_LEAF_BIT != 0
    }

    /// Check if a unit corresponds to a leaf or not.
    #[inline(always)]
    pub const fn value(&self) -> DictValue {
        DictValue(self.0 & VALUE_BITS)
    }

    /// Read a label with a leaf flag from a non-leaf unit.
    #[inline(always)]
    pub const fn label(&self) -> DictLabel {
        DictLabel(self.0 & LABEL_BITS)
    }

    /// Read an offset to child units from a non-leaf unit.
    #[inline(always)]
    pub const fn offset(&self) -> DictOffset {
        // Data from higher 22 bits.
        let base = self.0 >> 10;
        // 8 if EXTENSION_BIT is set, 0 otherwise.
        let extention = (self.0 & EXTENSION_BIT) >> 6;
        // Multiply by 256 if EXTENSION_BIT is set.
        DictOffset(base << extention)
    }
}

impl fmt::LowerHex for DictEntry {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
