use super::super::Gram;
use super::super::GramReg;
use super::GramRegistrySuspended;
use super::LatMapEntry;
use crate::model::dictionary::data_format::binpack::Suspend;

#[derive(Debug, Clone, Copy)]
pub struct GramRegistry<'d> {
    pub(super) reg_map: &'d [GramReg],
    pub(super) lat_map: &'d [LatMapEntry],
}

impl<'d> GramRegistry<'d> {
    #[inline(always)]
    pub(crate) fn new(reg_map: &'d [GramReg], lat_map: &'d [LatMapEntry]) -> Self {
        GramRegistry { reg_map, lat_map }
    }

    #[inline(always)]
    pub(crate) fn into_inner(self) -> (&'d [GramReg], &'d [LatMapEntry]) {
        (self.reg_map, self.lat_map)
    }

    #[inline]
    pub fn find_gram(&self, gram_int: &str) -> Option<Gram> {
        let idx = self
            .lat_map
            .binary_search_by(|probe| probe.id.as_str().cmp(gram_int))
            .ok()?;
        Some(self.lat_map[idx].gram)
    }

    #[inline(always)]
    pub fn find_reg_by_id(&self, gram_int: &str) -> Option<&'d GramReg> {
        self.get_reg_by_gram(self.find_gram(gram_int)?)
    }

    #[inline(always)]
    pub fn get_reg_by_gram(&self, gram: Gram) -> Option<&'d GramReg> {
        self.reg_map.get(gram.as_index())
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &'d GramReg> {
        self.reg_map.iter()
    }
}

impl<'d> Suspend for GramRegistry<'d> {
    type Suspended = GramRegistrySuspended;

    fn suspend(&self) -> Self::Suspended {
        GramRegistrySuspended {
            reg_map: self.reg_map as *const _,
            lat_map: self.lat_map as *const _,
        }
    }
}
