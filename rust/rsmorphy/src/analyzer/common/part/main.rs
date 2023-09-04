use std::fmt;

use crate::analyzer::common::Affix;
use crate::analyzer::common::Base;
use crate::model::Dictionary;

#[derive(Debug, Clone)]
pub struct Main {
    pub base: Base,
    pub affix: Option<Affix>,
}

impl Main {
    #[inline(always)]
    pub fn new(base: impl Into<Base>, affix: Option<Affix>) -> Self {
        let base = base.into();
        Main { base, affix }
    }

    #[inline(always)]
    pub fn is_known(&self) -> bool {
        self.base.is_known()
            && self
                .affix
                .as_ref()
                .map(|affix| affix.is_known())
                .unwrap_or(true)
    }

    #[inline(always)]
    pub fn write_word_lower(&self, writer: &mut dyn fmt::Write) -> fmt::Result {
        if let Some(affix) = &self.affix {
            if !affix.is_known_suffix() {
                writer.write_str(&affix.affix)?;
            }
        }
        self.base.write_word_lower(writer)?;
        Ok(())
    }

    #[inline(always)]
    pub fn write_normal_form(&self, writer: &mut dyn fmt::Write, dict: &Dictionary) -> fmt::Result {
        if let Some(affix) = &self.affix {
            if !affix.is_known_suffix() {
                writer.write_str(&affix.affix)?;
            }
        }
        self.base.write_normal_form(writer, dict)?;
        Ok(())
    }
}
