#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub is_known: bool,
    pub word: String,
}

impl Word {
    pub fn new<W: Into<String>>(word: W, is_known: bool) -> Self {
        Word {
            word: word.into(),
            is_known,
        }
    }
}
