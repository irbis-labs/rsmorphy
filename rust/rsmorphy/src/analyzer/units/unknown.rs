use crate::analyzer::context::ParseContext;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Score;
use crate::analyzer::model::Unknown;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::OpencorporaTagReg;

const SCORE: Score = Score::Fake(1.0);

#[derive(Debug, Clone)]
pub struct UnknownAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for UnknownAnalyzer {
    fn default() -> Self {
        UnknownAnalyzer {
            tag: OpencorporaTagReg::from_fmt_int("UNKN"),
        }
    }
}

impl AnalyzerUnit for UnknownAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("UnknownAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        if ctx.seen_parses().is_empty() {
            let lex = Lex::from_stack(StackSource::from(Unknown::new(ctx.word_lower())));
            ctx.push(Parsed::new(lex, SCORE));
        }
    }
}
