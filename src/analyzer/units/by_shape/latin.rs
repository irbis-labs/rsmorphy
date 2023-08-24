use crate::analyzer::context::ParseContext;
use crate::analyzer::model::abc::*;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Shaped;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::OpencorporaTagReg;
use crate::shapes::is_latin;

/// This analyzer marks latin words with "LATN" tag.
///
/// Example: "pdf" -> LATN

#[derive(Debug, Clone)]
pub struct LatinAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for LatinAnalyzer {
    fn default() -> Self {
        LatinAnalyzer {
            tag: OpencorporaTagReg::from_fmt_int("LATN"),
        }
    }
}

impl AnalyzerUnit for LatinAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(trace_analyzer)]
        log::trace!("LatinAnalyzer::parse()");
        #[cfg(trace_analyzer)]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        if !is_latin(ctx.word_lower()) {
            return;
        }
        let shaped = Shaped::latin(ctx.word_lower());
        let score = shaped.score();
        let lex = Lex::from_stack(StackSource::from(shaped));
        ctx.push(Parsed::new(lex, score));
    }
}
