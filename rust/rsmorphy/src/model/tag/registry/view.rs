use crate::model::dictionary::data_format::binpack::Suspend;
use crate::model::tag::registry::suspended::TagRegistrySuspended;
use crate::model::tag::TagReg;

#[derive(Debug, Clone, Copy)]
pub struct TagRegistry<'d> {
    pub(super) tags: &'d [TagReg],
}

impl<'d> TagRegistry<'d> {
    #[inline(always)]
    pub fn new(tags: &'d [TagReg]) -> Self {
        TagRegistry { tags }
    }

    #[inline(always)]
    pub fn into_inner(self) -> &'d [TagReg] {
        self.tags
    }

    #[inline(always)]
    pub fn get(&self, id: u16) -> Option<&'d TagReg> {
        self.tags.get(id as usize)
    }
}

impl<'d> Suspend for TagRegistry<'d> {
    type Suspended = TagRegistrySuspended;

    fn suspend(&self) -> Self::Suspended {
        TagRegistrySuspended {
            tags: self.tags as *const _,
        }
    }
}
