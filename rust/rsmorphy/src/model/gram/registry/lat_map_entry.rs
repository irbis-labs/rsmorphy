use byteorder::ByteOrder;

use super::super::Gram;
use super::super::GramId;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub(crate) struct LatMapEntry {
    pub(super) id: GramId,
    pub(super) gram: Gram,
    pub(super) _reserved: [u8; 3],
}

impl LatMapEntry {
    #[inline(always)]
    pub fn into_order<Order: ByteOrder>(self) -> Self {
        // Currently, it is noop.
        self
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(mem::size_of::<LatMapEntry>(), 16);
    }
}
