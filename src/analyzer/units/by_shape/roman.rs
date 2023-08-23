use crate::analyzer::context::ParseContext;
use crate::analyzer::unit::AnalyzerUnit;
use crate::container::abc::*;
use crate::container::stack::StackSource;
use crate::container::Lex;
use crate::container::Parsed;
use crate::container::Shaped;
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
        #[cfg(trace_analyzer)]
        log::trace!("RomanAnalyzer::parse()");
        #[cfg(trace_analyzer)]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        let word_lower = ctx.word_lower();
        if !is_roman_number(word_lower) {
            return;
        }
        let shaped = Shaped::roman_number(word_lower);
        let score = shaped.score();
        let lex = Lex::from_stack(ctx.morph(), StackSource::from(shaped));
        ctx.push(Parsed::new(lex, score));
    }
}
