use std::mem;
use std::ops;

const _SIZE_ASSERT: () = {
    assert!(BinChunk::SIZE == mem::size_of::<BinChunk>());
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct BinChunk {
    data: [u8; Self::SIZE],
}

impl BinChunk {
    pub const SIZE: usize = 64;

    #[inline(always)]
    pub fn zeroed() -> Self {
        BinChunk::new([0; Self::SIZE])
    }

    #[inline(always)]
    pub fn new(data: [u8; Self::SIZE]) -> Self {
        BinChunk { data }
    }

    #[inline(always)]
    pub(super) fn copy_from_slice_unchecked(slice: &[u8]) -> Self {
        debug_assert_eq!(slice.len(), Self::SIZE);

        let data = slice.try_into().unwrap();
        BinChunk { data }
    }
}

impl ops::Deref for BinChunk {
    type Target = [u8; Self::SIZE];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl ops::DerefMut for BinChunk {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
