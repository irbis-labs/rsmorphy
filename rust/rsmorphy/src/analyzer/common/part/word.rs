use inlinable_string::InlinableString;

#[derive(Debug, Clone, PartialEq)]
pub struct WordStruct {
    pub is_known: bool,
    pub word: InlinableString,
}

impl WordStruct {
    #[inline(always)]
    pub fn new(word: impl Into<InlinableString>, is_known: bool) -> Self {
        let word = word.into();
        WordStruct { word, is_known }
    }

    #[inline(always)]
    pub fn known(word: impl Into<InlinableString>) -> Self {
        WordStruct::new(word, true)
    }

    #[inline(always)]
    pub fn unknown(word: impl Into<InlinableString>) -> Self {
        WordStruct::new(word, false)
    }
}
