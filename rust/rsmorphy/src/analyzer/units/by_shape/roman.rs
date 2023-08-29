use crate::analyzer::context::ParseContext;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Shaped;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::tag::TagReg;
use crate::shapes::is_roman_number;
use crate::Score;

const SCORE: Score = Score::Fake(0.9);

#[derive(Debug, Clone)]
pub struct RomanAnalyzer {
    pub tag: TagReg,
    score: Score,
}

impl RomanAnalyzer {
    pub(in crate::analyzer) fn new(grammemes: &GramRegistry) -> Self {
        RomanAnalyzer {
            tag: TagReg::from_fmt_int(grammemes, "ROMN"),
            score: SCORE,
        }
    }
}

impl AnalyzerUnit for RomanAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("RomanAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        let word_lower = ctx.word_lower();
        if !is_roman_number(word_lower) {
            return;
        }
        let shaped = Shaped::roman_number(word_lower);
        let lex = Lex::from_stack(StackSource::from(shaped));
        ctx.push(Parsed::new(lex, self.score));
    }
}
