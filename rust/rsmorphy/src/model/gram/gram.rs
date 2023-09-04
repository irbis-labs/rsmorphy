use std::num::NonZeroU8;

use crate::model::gram::set::GramSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Gram {
    id: NonZeroU8,
}

impl Gram {
    #[cfg(test)]
    pub(super) fn new(id: u8) -> Gram {
        unsafe { Self::new_unchecked(id) }
    }

    #[inline(always)]
    pub(super) unsafe fn new_unchecked(id: u8) -> Self {
        debug_assert!(id > 0 && id < 128, "Gram id is out of range 1..128: {}", id);
        let id = NonZeroU8::new_unchecked(id);
        Self { id }
    }

    #[inline(always)]
    pub fn id(&self) -> u8 {
        self.id.get()
    }

    #[inline(always)]
    pub fn as_set(self) -> GramSet {
        GramSet::from(self)
    }
}
