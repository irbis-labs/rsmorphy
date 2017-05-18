use super::Lex;
use super::Score;


#[derive(Debug, Clone, PartialEq)]
pub struct Parsed {
    pub lex: Lex,
    pub score: Score,
}

pub type ParseResult = Vec<Parsed>;
