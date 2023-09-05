use std::ops;

use super::guide_entry::GuideEntry;
use crate::dawg::dict_entry::DictIndex;

#[derive(Debug, Clone, Copy)]
pub struct DawgGuide<'d> {
    units: &'d [GuideEntry],
}

impl<'d> DawgGuide<'d> {
    pub fn new(units: &'d [GuideEntry]) -> Self {
        DawgGuide { units }
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
