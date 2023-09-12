use super::GramRegistry;
use super::GramRegistryBuilder;
use super::LatMapEntry;
use crate::types::gram::Gram;
use crate::types::gram::GramId;
use crate::types::gram::GramReg;

#[derive(Debug, Clone)]
pub struct GramRegistryStorage {
    reg_map: Box<[GramReg]>,
    lat_map: Box<[LatMapEntry]>,
}

impl GramRegistryStorage {
    pub fn builder() -> GramRegistryBuilder {
        GramRegistryBuilder::new()
    }

    pub(super) fn from_builder(
        reg_map: Box<[GramReg]>,
        lat_map: impl IntoIterator<Item = (GramId, Gram)>,
    ) -> Self {
        let mut lat_map: Box<_> = lat_map
            .into_iter()
            .map(|(id, gram)| LatMapEntry {
                id,
                gram,
                _reserved: [0; 3],
            })
            .collect();
        lat_map.sort_by_key(|entry| entry.id);
        GramRegistryStorage { reg_map, lat_map }
    }

    #[inline(always)]
    pub fn to_ref(&self) -> GramRegistry {
        GramRegistry::new(&self.reg_map, &self.lat_map)
    }
}
