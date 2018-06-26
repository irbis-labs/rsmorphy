use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct WordStruct {
    is_known: bool,
    word: Arc<String>,
}

impl WordStruct {
    pub fn new<W: Into<String>>(word: W, is_known: bool) -> Self {
        let word = Arc::new(word.into());
        WordStruct { word, is_known }
    }

    pub fn known<W: Into<String>>(word: W) -> Self {
        WordStruct::new(word, true)
    }

    pub fn unknown<W: Into<String>>(word: W) -> Self {
        WordStruct::new(word, false)
    }

    pub fn is_known(&self) -> bool {
        self.is_known
    }

    pub fn word(&self) -> &str {
        &self.word
    }
}
