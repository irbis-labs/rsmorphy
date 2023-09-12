use std::ops;

use super::dict_entry::DictIndex;
use super::guide_entry::GuideEntry;
use crate::cryostasis::Revive;
use crate::cryostasis::Suspend;

#[derive(Debug, Clone, Copy)]
pub struct DawgGuide<'d> {
    units: &'d [GuideEntry],
}

#[derive(Debug)]
pub struct DawgGuideSuspended {
    units: *const [GuideEntry],
}

impl Suspend for DawgGuide<'_> {
    type Suspended = DawgGuideSuspended;

    #[inline(always)]
    fn suspend(&self) -> Self::Suspended {
        DawgGuideSuspended {
            units: self.units as *const _,
        }
    }
}

impl Revive for DawgGuideSuspended {
    type Revived<'d> = DawgGuide<'d>;

    #[inline(always)]
    unsafe fn revive<'d>(&self, _source_bin: &'d [u8]) -> Self::Revived<'d> {
        DawgGuide {
            units: unsafe { &*self.units },
        }
    }
}

impl<'d> DawgGuide<'d> {
    pub fn new(units: &'d [GuideEntry]) -> Self {
        DawgGuide { units }
    }

    pub fn into_inner(self) -> &'d [GuideEntry] {
        self.units
    }
}

impl<'d> ops::Index<DictIndex> for DawgGuide<'d> {
    type Output = GuideEntry;

    #[inline(always)]
    fn index(&self, index: DictIndex) -> &Self::Output {
        assert!(index.as_usize() < self.units.len(), "index out of bounds");
        unsafe { self.units.get_unchecked(index.as_usize()) }
    }
}
