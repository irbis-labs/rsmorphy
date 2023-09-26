use smart_string::SmartString;

#[derive(Debug, Clone, PartialEq)]
pub struct WordStruct {
    pub is_known: bool,
    pub word: SmartString,
}

impl WordStruct {
    #[inline(always)]
    pub fn new(word: impl Into<SmartString>, is_known: bool) -> Self {
        let word = word.into();
        WordStruct { word, is_known }
    }

    #[inline(always)]
    pub fn known(word: impl Into<SmartString>) -> Self {
        WordStruct::new(word, true)
    }

    #[inline(always)]
    pub fn unknown(word: impl Into<SmartString>) -> Self {
        WordStruct::new(word, false)
    }
}
