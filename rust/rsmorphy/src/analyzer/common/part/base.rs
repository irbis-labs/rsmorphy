use std::fmt;

use derive_more::From;

use crate::analyzer::common::part::initial::Initial;
use crate::analyzer::common::DictionaryWord;
use crate::analyzer::common::HyphenAdverb;
use crate::analyzer::common::Shaped;
use crate::analyzer::common::Unknown;
use crate::model::dictionary::ParadigmId;
use crate::model::Dictionary;

#[derive(Debug, Clone, From)]
pub enum Base {
    Dictionary(DictionaryWord),
    HyphenAdverb(HyphenAdverb),
    Initial(Initial),
    Shaped(Shaped),
    Unknown(Unknown),
}

impl Base {
    #[inline(always)]
    pub fn is_known(&self) -> bool {
        match self {
            Base::Dictionary(dw) => dw.word_lower.is_known,
            Base::HyphenAdverb(ha) => ha.word_lower.is_known,
            Base::Initial(_) | Base::Shaped(_) | Base::Unknown(_) => false,
        }
    }

    #[inline(always)]
    pub fn paradigm_id(&self) -> Option<ParadigmId> {
        match self {
            Base::Dictionary(dw) => Some(dw.para_id),
            Base::HyphenAdverb(_) | Base::Initial(_) | Base::Shaped(_) | Base::Unknown(_) => None,
        }
    }

    pub fn write_word_lower(&self, writer: &mut dyn fmt::Write) -> fmt::Result {
        match self {
            Base::Dictionary(dw) => writer.write_str(&dw.word_lower.word)?,
            Base::HyphenAdverb(ha) => writer.write_str(&ha.word_lower.word)?,
            Base::Initial(initial) => writer.write_str(&initial.letter)?,
            Base::Shaped(shaped) => writer.write_str(&shaped.word)?,
            Base::Unknown(unknown) => writer.write_str(&unknown.word_lower)?,
        }
        Ok(())
    }

    pub fn write_normal_form(&self, writer: &mut dyn fmt::Write, dict: &Dictionary) -> fmt::Result {
        match self {
            Base::Dictionary(dw) => {
                dict.write_normal_form(writer, dw.para_id, dw.idx, &dw.word_lower.word)?
            }
            Base::HyphenAdverb(ha) => writer.write_str(&ha.word_lower.word)?,
            Base::Initial(initial) => writer.write_str(&initial.letter)?,
            Base::Shaped(shaped) => writer.write_str(&shaped.word)?,
            Base::Unknown(unknown) => writer.write_str(&unknown.word_lower)?,
        }
        Ok(())
    }
}
