use inlinable_string::InlinableString;

#[derive(Debug, Clone, PartialEq)]
pub struct Unknown {
    pub word_lower: InlinableString,
}

impl Unknown {
    #[inline(always)]
    pub fn new(word_lower: impl Into<InlinableString>) -> Self {
        let word_lower = word_lower.into();
        Unknown { word_lower }
    }
}
