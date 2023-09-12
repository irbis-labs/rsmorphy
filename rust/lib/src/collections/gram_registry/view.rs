use pascal_string::PascalString;

use super::GramRegistrySuspended;
use super::LatMapEntry;
use crate::cryostasis::Suspend;
use crate::types::gram::Gram;
use crate::types::gram::GramReg;
use crate::types::gram::GramSet;
use crate::types::tag::TagReg;

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

    // TODO replace panic with Result
    pub fn parse_tag(&self, fmt_int: &str) -> TagReg {
        let mut grammemes = GramSet::empty();

        #[cfg(feature = "log_trace_model")]
        log::trace!("tags for {}", fmt_int);

        for gram_int in fmt_int.split(|c| matches!(c, ',' | ' ')) {
            let Some(gram) = self.find_gram(gram_int) else {
                panic!("Unknown gram: {}", gram_int);
            };
            grammemes.insert(gram);
        }

        let Ok(fmt_int) = PascalString::try_from(fmt_int) else {
            panic!(
                "Too long tag {} ({} exceeds {} bytes)",
                fmt_int,
                fmt_int.len(),
                TagReg::FMT_INT_MAX_LEN
            );
        };

        TagReg { fmt_int, grammemes }
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
