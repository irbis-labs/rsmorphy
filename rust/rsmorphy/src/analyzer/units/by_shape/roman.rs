use crate::analyzer::context::ParseContext;
use crate::analyzer::model::abc::*;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Shaped;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::OpencorporaTagReg;
use crate::shapes::is_roman_number;

#[derive(Debug, Clone)]
pub struct RomanAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for RomanAnalyzer {
    fn default() -> Self {
        RomanAnalyzer {
            tag: OpencorporaTagReg::from_fmt_int("ROMN"),
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
        let score = shaped.score();
        let lex = Lex::from_stack(StackSource::from(shaped));
        ctx.push(Parsed::new(lex, score));
    }
}
