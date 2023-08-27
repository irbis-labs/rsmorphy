use std::collections::HashMap;

use inlinable_string::InlinableString;
use serde::Deserialize;

use super::Gram;
use super::GramId;
use super::GramReg;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::gram::GramRegistryError;
use crate::opencorpora::gram::GramRegistryError::ParentNotFound;
use crate::opencorpora::gram::GramSet;
use crate::opencorpora::tag::TagSet;

#[derive(Debug)]
pub struct GramRegistryBuilder {
    reg_map: Vec<GramReg>,
    lat_map: HashMap<GramId, Gram>,
    /// maps child to parent
    parent_map: HashMap<Gram, GramId>,
}

#[derive(Debug, Deserialize)]
pub struct GramRow {
    name: InlinableString,
    parent: InlinableString,
    alias: InlinableString,
    desciption: InlinableString,
}

impl GramRow {
    pub fn new(name: &str, parent: &str, alias: &str, desciption: &str) -> Self {
        GramRow {
            name: name.into(),
            parent: parent.into(),
            alias: alias.into(),
            desciption: desciption.into(),
        }
    }
}

impl GramRegistryBuilder {
    pub(super) fn new() -> Self {
        GramRegistryBuilder {
            reg_map: Vec::new(),
            lat_map: HashMap::new(),
            parent_map: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn add_grammeme(&mut self, row: GramRow, extra: bool) -> Result<Gram, GramRegistryError> {
        self.add_grammeme_internal(&row.name, &row.parent, row.alias, row.desciption, extra)
    }

    #[inline(always)]
    fn add_grammeme_internal(
        &mut self,
        name: impl AsRef<str>,
        parent: impl AsRef<str>,
        alias: impl Into<InlinableString>,
        description: impl Into<InlinableString>,
        extra: bool,
    ) -> Result<Gram, GramRegistryError> {
        self.add_grammeme_internal_(
            name.as_ref(),
            parent.as_ref(),
            alias.into(),
            description.into(),
            extra,
        )
    }

    fn add_grammeme_internal_(
        &mut self,
        name: &str,
        parent: &str,
        alias: InlinableString,
        description: InlinableString,
        extra: bool,
    ) -> Result<Gram, GramRegistryError> {
        #[cfg(feature = "trace_opencorpora")]
        log::trace!(
            "add grammeme: name = {}, parent = {}, alias = {}, description = {}, extra = {}",
            name,
            parent,
            alias,
            description,
            extra
        );

        let parent = match parent {
            "" => None,
            _ => Some(GramId::from_str(parent)?),
        };

        let name = GramId::from_str(name)?;

        let gram = self.reg_map.len() + 1;
        if gram == 128 {
            return Err(GramRegistryError::RegistryIsFull(name));
        }
        // SAFETY: Here is an authoritative source of all grams. Gram has index, that is
        //   about to be put in registry. It shouldn't be used before it is put in registry.
        let gram = unsafe { Gram::new_unchecked(gram as u8) };

        // All updates in this block should be executed atomically to keep registry consistent.
        {
            if let Some(parent) = parent {
                self.parent_map.insert(gram, parent);
            }

            let reg = GramReg {
                gram,
                parent: None,
                name,
                alias: alias.into(),
                description: description.into(),
                children: GramSet::new(),
                incompatible: GramSet::new(),
                tag: TagSet::from_gram_id(name),
                extra,
            };
            self.reg_map.push(reg);
            self.lat_map.insert(name, gram);
        }

        Ok(gram)
    }

    pub fn finish(mut self) -> Result<GramRegistry, GramRegistryError> {
        //
        // Add extra grammemes
        //

        let extra = [
            ("PNCT", "", "ЗПР", "Знак пунктуации"),
            ("LATN", "", "ЛАТ", "Латиница"),
            ("NUMB", "", "ЧИСЛО", "Число"),
            ("intg", "NUMB", "цел", "Целое"),
            ("real", "NUMB", "вещ", "Вещественное"),
            ("ROMN", "", "РИМ", "Римские цифры"),
            ("UNKN", "", "НЕИЗВ", "Неизвестная словоформа"),
        ];

        for (name, parent, alias, description) in extra {
            self.add_grammeme_internal(name, parent, alias, description, true)?;
        }

        //

        let GramRegistryBuilder {
            mut reg_map,
            lat_map,
            parent_map,
        } = self;

        //
        // Fill parents and children
        //

        for (child_gram, parent_gram_id) in parent_map {
            let parent_gram = *lat_map
                .get(&parent_gram_id)
                .ok_or_else(|| ParentNotFound(parent_gram_id.as_str().into()))?;
            let parent_reg = &mut reg_map[parent_gram.id() as usize - 1];
            parent_reg.children.insert(child_gram);

            let child = &mut reg_map[child_gram.id() as usize - 1];
            child.parent = Some(parent_gram);
        }

        //
        // Fill incompatible
        //

        // Extra incompatible:   'plur': set(['GNdr'])
        // {u'plur': set([u'GNdr', u'masc', u'femn', u'neut'])}

        let find_gram = |name: &str| {
            lat_map
                .get(name)
                .copied()
                .ok_or_else(|| ParentNotFound(name.into()))
        };

        let plur = find_gram("plur")?;
        let gndr = find_gram("GNdr")?;
        let extra_incompatible = gndr.as_set() | reg_map[gndr.id() as usize - 1].children;

        // TODO: check if this is correct:
        //   `gndr children minus plur` and `gram_reg children minus gram_reg itself`
        //   looks senseless.

        let plur_reg = &mut reg_map[plur.id() as usize - 1];
        plur_reg.incompatible |= extra_incompatible - plur.as_set();

        for gram_reg in &mut reg_map {
            gram_reg.incompatible |= gram_reg.children - gram_reg.gram.as_set();
        }

        //

        #[cfg(feature = "trace_opencorpora")]
        log::trace!("GramRegistryBuilder::finish: reg_map = {:?}", reg_map);

        Ok(GramRegistry::from_builder(reg_map, lat_map))
    }
}
