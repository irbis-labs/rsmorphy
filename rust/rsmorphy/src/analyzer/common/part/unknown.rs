use smart_string::SmartString;

#[derive(Debug, Clone, PartialEq)]
pub struct Unknown {
    pub word_lower: SmartString,
}

impl Unknown {
    #[inline(always)]
    pub fn new(word_lower: impl Into<SmartString>) -> Self {
        let word_lower = word_lower.into();
        Unknown { word_lower }
    }
}
