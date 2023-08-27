use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use super::Gram;
use super::GramId;
use super::GramReg;
use crate::opencorpora::gram::registry_builder::GramRegistryBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct GramRegistry {
    reg_map: Vec<GramReg>,
    lat_map: HashMap<GramId, Gram>,
}

impl GramRegistry {
    pub fn builder() -> GramRegistryBuilder {
        GramRegistryBuilder::new()
    }

    pub(super) fn from_builder(reg_map: Vec<GramReg>, lat_map: HashMap<GramId, Gram>) -> Self {
        GramRegistry { reg_map, lat_map }
    }

    #[inline(always)]
    pub fn get_gram(&self, gram_int: GramId) -> Gram {
        // SAFETY: GramId can exist only if it is in registry.
        self.lat_map.get(&gram_int).copied().unwrap()
    }

    #[inline(always)]
    pub fn find_gram(&self, gram_int: &str) -> Option<Gram> {
        self.lat_map.get(gram_int).copied()
    }

    #[inline(always)]
    pub fn get_reg_by_id(&self, gram_int: GramId) -> &GramReg {
        self.get_reg_by_gram(self.get_gram(gram_int))
    }

    #[inline(always)]
    pub fn get_reg_by_gram(&self, gram: Gram) -> &GramReg {
        let idx = gram.id() as usize - 1;
        // SAFETY: Gram can exist only if it is in registry.
        unsafe { self.reg_map.get_unchecked(idx) }
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &GramReg> {
        self.reg_map.iter()
    }
}
