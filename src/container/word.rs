use inlinable_string::InlinableString;

#[derive(Debug, Clone, PartialEq)]
pub struct WordStruct {
    is_known: bool,
    word: InlinableString,
}

impl WordStruct {
    pub fn new<W: Into<InlinableString>>(word: W, is_known: bool) -> Self {
        let word = word.into();
        WordStruct { word, is_known }
    }

    pub fn known<W: Into<InlinableString>>(word: W) -> Self {
        WordStruct::new(word, true)
    }

    pub fn unknown<W: Into<InlinableString>>(word: W) -> Self {
        WordStruct::new(word, false)
    }

    pub fn is_known(&self) -> bool {
        self.is_known
    }

    pub fn word(&self) -> &str {
        &self.word
    }
}
