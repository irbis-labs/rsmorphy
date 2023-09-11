use std::num::NonZeroU8;

use crate::model::gram::set::GramSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct Gram {
    id: NonZeroU8,
}

impl Gram {
    pub(crate) fn new(id: u8) -> Gram {
        assert!(id > 0 && id < 128, "Gram id is out of range 1..128: {}", id);
        unsafe { Self::new_unchecked(id) }
    }

    #[inline(always)]
    pub(crate) unsafe fn new_unchecked(id: u8) -> Self {
        debug_assert!(id > 0 && id < 128, "Gram id is out of range 1..128: {}", id);
        let id = NonZeroU8::new_unchecked(id);
        Self { id }
    }

    #[inline(always)]
    pub fn into_inner(self) -> u8 {
        self.id.into()
    }

    #[inline(always)]
    pub fn id(&self) -> u8 {
        self.id.get()
    }

    #[inline(always)]
    pub fn as_index(&self) -> usize {
        self.id() as usize - 1
    }

    #[inline(always)]
    pub fn as_set(self) -> GramSet {
        GramSet::from(self)
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(mem::size_of::<Gram>(), 1);
    }
}
