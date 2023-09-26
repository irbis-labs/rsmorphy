use smart_string::SmartString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShapeKind {
    IntegerNumber,
    FractionalNumber,
    RomanNumber,
    Latin,
    Punctuation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shaped {
    pub word: SmartString,
    pub kind: ShapeKind,
}

impl Shaped {
    #[inline(always)]
    pub fn new(word: impl Into<SmartString>, kind: ShapeKind) -> Self {
        let word = word.into();
        Shaped { word, kind }
    }

    #[inline(always)]
    pub fn integer(word: impl Into<SmartString>) -> Self {
        Shaped::new(word, ShapeKind::IntegerNumber)
    }

    #[inline(always)]
    pub fn fractional(word: impl Into<SmartString>) -> Self {
        Shaped::new(word, ShapeKind::FractionalNumber)
    }

    #[inline(always)]
    pub fn roman_number(word: impl Into<SmartString>) -> Self {
        Shaped::new(word, ShapeKind::RomanNumber)
    }

    #[inline(always)]
    pub fn latin(word: impl Into<SmartString>) -> Self {
        Shaped::new(word, ShapeKind::Latin)
    }

    #[inline(always)]
    pub fn punctuation(word: impl Into<SmartString>) -> Self {
        Shaped::new(word, ShapeKind::Punctuation)
    }
}
