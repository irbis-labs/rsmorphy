use inlinable_string::InlinableString;

#[derive(Debug, Clone, PartialEq)]
pub struct WordStruct {
    pub(in crate::analyzer) is_known: bool,
    pub(in crate::analyzer) word: InlinableString,
}

impl WordStruct {
    #[inline(always)]
    pub fn new<W: Into<InlinableString>>(word: W, is_known: bool) -> Self {
        let word = word.into();
        WordStruct { word, is_known }
    }

    #[inline(always)]
    pub fn known<W: Into<InlinableString>>(word: W) -> Self {
        WordStruct::new(word, true)
    }

    #[inline(always)]
    pub fn unknown<W: Into<InlinableString>>(word: W) -> Self {
        WordStruct::new(word, false)
    }

    #[inline(always)]
    pub fn is_known(&self) -> bool {
        self.is_known
    }

    #[inline(always)]
    pub fn word(&self) -> &str {
        &self.word
    }

    #[inline(always)]
    pub fn into_parts(self) -> (InlinableString, bool) {
        (self.word, self.is_known)
    }
}
