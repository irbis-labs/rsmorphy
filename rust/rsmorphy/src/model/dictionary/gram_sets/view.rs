use std::mem;

use super::owned::NON_PORDUCTIVE_GRAMSET_INDEX;
use super::owned::NUMERAL_GRAMSETS_LEN;
use super::GramSetsOwned;
use super::GramSetsSuspended;
use super::NumeralGrammemes;
use crate::model::dictionary::data_format::binpack::util::bytes_to_ref;
use crate::model::dictionary::data_format::binpack::util::convert_slice;
use crate::model::dictionary::data_format::binpack::Suspend;
use crate::GramSet;

#[derive(Debug, Clone, Copy)]
pub struct GramSets<'a> {
    pub(super) sets: &'a GramSetsOwned,
}

impl<'a> GramSets<'a> {
    #[inline(always)]
    pub fn new(sets: &'a GramSetsOwned) -> Self {
        Self { sets }
    }

    #[inline(always)]
    pub fn into_inner(self) -> &'a [GramSet] {
        &self.sets.sets
    }

    #[inline(always)]
    pub fn numeral(&self) -> &NumeralGrammemes {
        debug_assert_eq!(
            mem::size_of::<NumeralGrammemes>(),
            mem::size_of::<GramSet>() * 10
        );
        let numeral = &self.sets.sets[0..NUMERAL_GRAMSETS_LEN];
        let bytes = unsafe { convert_slice(numeral) }.unwrap();
        unsafe { bytes_to_ref(bytes) }
    }

    #[inline(always)]
    pub fn non_productive(&self) -> GramSet {
        self.sets.sets[NON_PORDUCTIVE_GRAMSET_INDEX]
    }
}

impl Suspend for GramSets<'_> {
    type Suspended = GramSetsSuspended;

    fn suspend(&self) -> Self::Suspended {
        GramSetsSuspended {
            sets: self.sets as *const _,
        }
    }
}
