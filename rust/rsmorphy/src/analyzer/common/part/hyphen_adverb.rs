use crate::analyzer::common::WordStruct;

#[derive(Debug, Clone, PartialEq)]
pub struct HyphenAdverb {
    pub word_lower: WordStruct,
}

impl HyphenAdverb {
    #[inline(always)]
    pub fn new(word_lower: WordStruct) -> Self {
        HyphenAdverb { word_lower }
    }

    #[inline(always)]
    pub fn is_lemma(&self) -> bool {
        true
    }

    #[inline(always)]
    pub fn is_known(&self) -> bool {
        self.word_lower.is_known
    }
}
