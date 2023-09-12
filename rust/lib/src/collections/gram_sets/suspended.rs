use super::GramSets;
use super::GramSetsOwned;
use crate::cryostasis::Revive;

#[derive(Debug, Clone)]
pub struct GramSetsSuspended {
    pub(super) sets: *const GramSetsOwned,
}

impl Revive for GramSetsSuspended {
    type Revived<'a> = GramSets<'a>;

    unsafe fn revive<'a>(&self, _source_bin: &'a [u8]) -> GramSets<'a> {
        GramSets {
            sets: unsafe { &*self.sets },
        }
    }
}
