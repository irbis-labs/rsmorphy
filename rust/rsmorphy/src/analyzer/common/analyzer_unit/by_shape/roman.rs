use std::marker::PhantomData;

use crate::analyzer::common::ParseContext;
use crate::analyzer::common::Shaped;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;
use crate::model::gram::GramRegistry;
use crate::model::tag::TagReg;
use crate::shapes::is_roman_number;

const SCORE: Score = Score::Fake(0.9);

#[derive(Debug, Clone)]
pub struct RomanAnalyzer<L: Lang> {
    pub tag: TagReg,
    pub score: Score,
    _lang: PhantomData<L>,
}

impl<L: Lang> RomanAnalyzer<L> {
    pub fn new(grammemes: &GramRegistry, score: Option<Score>) -> Self {
        RomanAnalyzer {
            tag: TagReg::from_fmt_int(grammemes, "ROMN"),
            score: score.unwrap_or(SCORE),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("RomanAnalyzer::parse(); {ctx:?}");

        let word = ctx.word();
        if !is_roman_number(word) {
            return;
        }
        let lex = Lex::from_base(Shaped::roman_number(word));
        ctx.push(Parse::new(lex, self.score));
    }
}
