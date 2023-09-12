#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PTWOptions {
    pub unique_words: u32,
    pub outcomes: u32,
    pub min_word_freq: u32,
}
