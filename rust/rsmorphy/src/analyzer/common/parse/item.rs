use crate::analyzer::common::Lex;
use crate::analyzer::common::Score;

#[derive(Debug, Clone)]
pub struct Parse {
    pub lex: Lex,
    pub score: Score,
}

impl Parse {
    #[inline(always)]
    pub fn new(lex: Lex, score: Score) -> Self {
        Parse { lex, score }
    }
}
