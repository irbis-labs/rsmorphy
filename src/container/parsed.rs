use super::Lex;
use super::Score;

#[derive(Debug, Clone, PartialEq)]
pub struct Parsed {
    pub lex: Lex,
    pub score: Score,
}

impl Parsed {
    pub fn new(lex: Lex, score: Score) -> Self {
        Parsed { lex, score }
    }
}

pub type ParseResult = Vec<Parsed>;
