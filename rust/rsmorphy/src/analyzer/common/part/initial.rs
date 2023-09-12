use lib::types::tag_kind::InitialKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Initial {
    pub letter: char,
    pub kind: InitialKind,
    pub tag_idx: u8,
}

impl Initial {
    #[inline(always)]
    pub fn new(letter: char, kind: InitialKind, tag_idx: u8) -> Self {
        Initial {
            letter,
            kind,
            tag_idx,
        }
    }
}
