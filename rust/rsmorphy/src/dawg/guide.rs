use std::ops;

use super::guide_entry::GuideEntry;
use crate::dawg::dict_entry::DictIndex;

#[derive(Debug, Clone)]
pub struct DawgGuide {
    units: Box<[GuideEntry]>,
}

impl DawgGuide {
    pub fn new(units: Box<[GuideEntry]>) -> Self {
        DawgGuide { units }
    }
}

impl ops::Index<DictIndex> for DawgGuide {
    type Output = GuideEntry;

    #[inline(always)]
    fn index(&self, index: DictIndex) -> &Self::Output {
        assert!(index.as_usize() < self.units.len(), "index out of bounds");
        unsafe { self.units.get_unchecked(index.as_usize()) }
    }
}
