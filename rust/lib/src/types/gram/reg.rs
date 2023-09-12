use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;

use byteorder::ByteOrder;
use smart_string::PascalString;

use super::Gram;
use super::GramId;
use super::GramSet;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(64))]
pub struct GramReg {
    pub name: GramId,         // 12 bytes   -\
    pub gram: Gram,           // 1 byte      |
    pub parent: Option<Gram>, // 1 byte      |
    pub extra: bool,          // 1 byte      |
    pub _reserved: [u8; 1],   // 1 byte     -/ = 16 bytes

    pub alias: PascalString<15>, // 16 bytes   -/ = 32 bytes
    pub children: GramSet,       // 16 bytes   -/ = 48 bytes
    pub incompatible: GramSet,   // 16 bytes   -/ = 64 bytes

    pub description: PascalString<63>, // 64 bytes   -/ = 128 bytes
}

impl GramReg {
    #[inline(always)]
    pub fn into_order<Order: ByteOrder>(self) -> Self {
        GramReg {
            name: self.name,
            gram: self.gram,
            parent: self.parent,
            extra: self.extra,
            _reserved: self._reserved,
            alias: self.alias,
            children: self.children.into_order::<Order>(),
            incompatible: self.incompatible.into_order::<Order>(),
            description: self.description,
        }
    }
}

impl PartialEq<Self> for GramReg {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.gram == other.gram
    }
}

impl Eq for GramReg {}

impl PartialOrd for GramReg {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.gram.partial_cmp(&other.gram)
    }
}

impl Ord for GramReg {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.gram.cmp(&other.gram)
    }
}

impl Hash for GramReg {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.gram.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(mem::size_of::<GramReg>(), 128);
    }
}
