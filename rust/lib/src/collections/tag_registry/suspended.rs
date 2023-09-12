use super::TagRegistry;
use crate::cryostasis::Revive;
use crate::types::tag::TagReg;

#[derive(Debug, Clone)]
pub struct TagRegistrySuspended {
    pub(super) tags: *const [TagReg],
}

impl Revive for TagRegistrySuspended {
    type Revived<'d> = TagRegistry<'d>;

    unsafe fn revive<'d>(&self, _source_bin: &'d [u8]) -> Self::Revived<'d> {
        TagRegistry { tags: &*self.tags }
    }
}
