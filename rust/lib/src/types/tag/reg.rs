use std::hash::Hash;
use std::hash::Hasher;
use std::mem;

use byteorder::ByteOrder;
use pascal_string::PascalString;

use crate::types::gram::GramSet;

const TAG_REG_SIZE: usize = 128;
const FMT_INT_MAX_LEN: usize = TAG_REG_SIZE - mem::size_of::<GramSet>() - 1;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(64))]
pub struct TagReg {
    pub fmt_int: PascalString<FMT_INT_MAX_LEN>,
    pub grammemes: GramSet,
}

impl TagReg {
    pub const FMT_INT_MAX_LEN: usize = FMT_INT_MAX_LEN;

    pub fn into_order<Order: ByteOrder>(self) -> Self {
        TagReg {
            fmt_int: self.fmt_int,
            grammemes: self.grammemes.into_order::<Order>(),
        }
    }
}

impl Hash for TagReg {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.grammemes.hash(state)
    }
}

impl PartialEq for TagReg {
    fn eq(&self, other: &TagReg) -> bool {
        self.grammemes.eq(&other.grammemes)
    }
}

impl Eq for TagReg {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(mem::size_of::<TagReg>(), TAG_REG_SIZE);
    }
}
