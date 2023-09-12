use super::LatMapEntry;
use crate::collections::gram_registry::GramRegistry;
use crate::cryostasis::Revive;
use crate::types::gram::GramReg;

#[derive(Debug, Clone, Copy)]
pub struct GramRegistrySuspended {
    pub(super) reg_map: *const [GramReg],
    pub(super) lat_map: *const [LatMapEntry],
}

impl Revive for GramRegistrySuspended {
    type Revived<'d> = GramRegistry<'d>;

    unsafe fn revive<'d>(&self, _source_bin: &'d [u8]) -> Self::Revived<'d> {
        GramRegistry {
            reg_map: &*self.reg_map,
            lat_map: &*self.lat_map,
        }
    }
}
