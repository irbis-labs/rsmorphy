use std::borrow::Borrow;
use std::ops;

use super::BinChunk;
use super::BinPackBuilder;
use crate::binpack::BinPackRef;
use crate::binutils::convert_slice;

/// A packed binary data, aligned to 64 bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinPack {
    pub(super) data: Box<[BinChunk]>,
    pub(super) len: usize,
}

impl BinPack {
    #[inline(always)]
    pub fn builder() -> BinPackBuilder {
        BinPackBuilder::new()
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        let bytes = unsafe { convert_slice(&self.data) }.unwrap();
        unsafe { bytes.get_unchecked(..self.len) }
    }
}

impl AsRef<[u8]> for BinPack {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ops::Deref for BinPack {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl FromIterator<u8> for BinPack {
    #[inline]
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut builder = BinPackBuilder::new();
        builder.extend(iter);
        builder.build()
    }
}

impl<'a> From<&'a [u8]> for BinPack {
    #[inline]
    fn from(bytes: &'a [u8]) -> Self {
        let mut builder = BinPackBuilder::new();
        builder.extend_from_bytes(bytes);
        builder.build()
    }
}

impl Borrow<BinPackRef> for BinPack {
    fn borrow(&self) -> &BinPackRef {
        BinPackRef::new(self)
    }
}
