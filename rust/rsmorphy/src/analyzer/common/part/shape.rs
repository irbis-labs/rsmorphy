use inlinable_string::InlinableString;

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
    pub word: InlinableString,
    pub kind: ShapeKind,
}

impl Shaped {
    #[inline(always)]
    pub fn new(word: impl Into<InlinableString>, kind: ShapeKind) -> Self {
        let word = word.into();
        Shaped { word, kind }
    }

    #[inline(always)]
    pub fn integer(word: impl Into<InlinableString>) -> Self {
        Shaped::new(word, ShapeKind::IntegerNumber)
    }

    #[inline(always)]
    pub fn fractional(word: impl Into<InlinableString>) -> Self {
        Shaped::new(word, ShapeKind::FractionalNumber)
    }

    #[inline(always)]
    pub fn roman_number(word: impl Into<InlinableString>) -> Self {
        Shaped::new(word, ShapeKind::RomanNumber)
    }

    #[inline(always)]
    pub fn latin(word: impl Into<InlinableString>) -> Self {
        Shaped::new(word, ShapeKind::Latin)
    }

    #[inline(always)]
    pub fn punctuation(word: impl Into<InlinableString>) -> Self {
        Shaped::new(word, ShapeKind::Punctuation)
    }
}
