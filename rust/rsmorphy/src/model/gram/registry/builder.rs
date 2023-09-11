use std::collections::HashMap;

use inlinable_string::InlinableString;
use serde::Deserialize;

use super::super::Gram;
use super::super::GramId;
use super::super::GramReg;
use crate::data_struct::pascal_string::PascalString;
use crate::model::gram::GramRegistryError;
use crate::model::gram::GramRegistryError::ParentNotFound;
use crate::model::gram::GramRegistryStorage;
use crate::model::gram::GramSet;

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
    parent: Option<InlinableString>,
    alias: InlinableString,
    desciption: InlinableString,
}

impl GramRow {
    pub fn new(name: &str, parent: &str, alias: &str, desciption: &str) -> Self {
        GramRow {
            name: name.into(),
            parent: Some(parent.into()),
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
        // Fix parent grammeme ids of Ukrainian pymorphy2 dict.
        let parent = Self::fix_ukrainian_parent(row.parent.as_deref());
        self.add_grammeme_internal(&row.name, parent, row.alias, row.desciption, extra)
    }

    #[inline(always)]
    fn fix_ukrainian_parent(parent: Option<&str>) -> &str {
        match parent.unwrap_or("") {
            "gndr" => "GNdr",
            "post" => "POST",
            "voice" => "VOic",
            "nmbr" => "NMbr",
            "trns" => "TRns",
            "tense" => "TEns",
            "mood" => "MOod",
            "aspc" => "ASpc",
            keep_other => keep_other,
        }
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
        #[cfg(feature = "log_trace_model")]
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

        let alias = PascalString::from_str_truncated(alias);
        let description = PascalString::from_str_truncated(description);

        let gram = self.reg_map.len() + 1;
        if gram == 128 {
            return Err(GramRegistryError::RegistryIsFull(name));
        }
        let gram = Gram::new(gram as u8);

        // All updates in this block should be executed atomically to keep registry consistent.
        {
            if let Some(parent) = parent {
                self.parent_map.insert(gram, parent);
            }

            let reg = GramReg {
                name,
                gram,
                parent: None,
                extra,
                _reserved: [0; 1],
                alias,
                children: GramSet::empty(),
                incompatible: GramSet::empty(),
                description,
            };
            self.reg_map.push(reg);
            self.lat_map.insert(name, gram);
        }

        Ok(gram)
    }

    pub fn build(mut self) -> Result<GramRegistryStorage, GramRegistryError> {
        //
        // Fix Ukrainian pymorphy2 dict
        //

        let missing = [("Name", "", "имя", "имя"), ("Patr", "", "отч", "отчество")];

        for (name, parent, alias, description) in missing {
            if !self.lat_map.contains_key(name) {
                self.add_grammeme_internal(name, parent, alias, description, false)?;
            }
        }

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

        #[cfg(feature = "log_trace_model")]
        log::trace!("GramRegistryBuilder::finish: reg_map = {:?}", reg_map);

        Ok(GramRegistryStorage::from_builder(reg_map.into(), lat_map))
    }
}
