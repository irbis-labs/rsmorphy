use crate::analyzer::model::Lex;
use crate::analyzer::model::Score;

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
