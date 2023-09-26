use std::hash::Hash;

use lib::collections::paradigm::ParadigmId;
use lib::types::tag::TagReg;
use smart_string::SmartString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seen<'a> {
    pub word: SmartString,
    pub tag: &'a TagReg,
    pub para_id: Option<ParadigmId>,
}

impl<'a> Seen<'a> {
    #[inline(always)]
    pub fn new(word: impl Into<SmartString>, tag: &'a TagReg, para_id: Option<ParadigmId>) -> Self {
        let word = word.into();
        Seen { word, tag, para_id }
    }
}
