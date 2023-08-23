use crate::analyzer::context::ParseContext;
use crate::analyzer::unit::AnalyzerUnit;
use crate::container::stack::StackSource;
use crate::container::Lex;
use crate::container::Parsed;
use crate::container::Score;
use crate::container::Unknown;
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
        #[cfg(trace_analyzer)]
        log::trace!("UnknownAnalyzer::parse()");
        #[cfg(trace_analyzer)]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        if ctx.seen_parses().is_empty() {
            let lex = Lex::from_stack(
                ctx.morph(),
                StackSource::from(Unknown::new(ctx.word_lower())),
            );
            ctx.push(Parsed::new(lex, SCORE));
        }
    }
}
