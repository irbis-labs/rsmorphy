use std::fmt;

use smart_string::SmartString;

use crate::analyzer::common::Lex;
use crate::analyzer::Lang;

#[derive(Debug, Clone, Copy)]
pub struct Token<'m, 'l, L: Lang> {
    pub morph: &'m L,
    pub lex: &'l Lex,
}

impl<'m, 'l, L: Lang> Token<'m, 'l, L> {
    pub fn new(morph: &'m L, lex: &'l Lex) -> Self {
        Token { morph, lex }
    }

    pub fn to_short_str(&self) -> SmartString {
        let mut enc = SmartString::new();
        self.morph.write_token(&mut enc, self.lex).unwrap();
        enc
    }
}

impl<'m, 'l, L: Lang> fmt::Display for Token<'m, 'l, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.morph.write_token(f, self.lex)
    }
}
