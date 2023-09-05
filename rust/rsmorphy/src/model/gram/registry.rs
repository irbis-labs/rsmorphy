use std::collections::HashMap;

use super::Gram;
use super::GramId;
use super::GramReg;
use crate::model::gram::registry_builder::GramRegistryBuilder;

#[derive(Debug, Clone, Copy)]
pub struct GramRegistry<'d> {
    reg_map: &'d [GramReg],
    lat_map: &'d HashMap<GramId, Gram>,
}

#[derive(Debug)]
pub struct GramRegistryStorage {
    reg_map: Box<[GramReg]>,
    lat_map: HashMap<GramId, Gram>,
}

impl GramRegistryStorage {
    #[inline(always)]
    pub fn to_ref(&self) -> GramRegistry {
        GramRegistry {
            reg_map: &self.reg_map,
            lat_map: &self.lat_map,
        }
    }
}

impl<'d> GramRegistry<'d> {
    pub fn builder() -> GramRegistryBuilder {
        GramRegistryBuilder::new()
    }

    pub(super) fn from_builder(
        reg_map: Box<[GramReg]>,
        lat_map: HashMap<GramId, Gram>,
    ) -> GramRegistryStorage {
        GramRegistryStorage { reg_map, lat_map }
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
