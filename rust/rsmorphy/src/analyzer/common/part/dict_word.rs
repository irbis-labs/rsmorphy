use lib::collections::paradigm::ParadigmId;
use lib::collections::paradigm::ParadigmIndex;

use crate::analyzer::common::WordStruct;

#[derive(Debug, Clone, PartialEq)]
pub struct DictionaryWord {
    pub word_lower: WordStruct,
    pub para_id: ParadigmId,
    pub idx: ParadigmIndex,
}

impl DictionaryWord {
    #[inline(always)]
    pub fn new(word_lower: WordStruct, para_id: ParadigmId, idx: ParadigmIndex) -> Self {
        DictionaryWord {
            word_lower,
            para_id,
            idx,
        }
    }
}
