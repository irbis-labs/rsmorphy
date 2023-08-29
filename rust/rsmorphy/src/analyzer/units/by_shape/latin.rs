use crate::analyzer::context::ParseContext;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Shaped;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::tag::TagReg;
use crate::shapes::is_latin;
use crate::Score;

const SCORE: Score = Score::Fake(0.9);

/// This analyzer marks latin words with "LATN" tag.
///
/// Example: "pdf" -> LATN

#[derive(Debug, Clone)]
pub struct LatinAnalyzer {
    pub tag: TagReg,
    score: Score,
}

impl LatinAnalyzer {
    pub(in crate::analyzer) fn new(grammemes: &GramRegistry) -> Self {
        LatinAnalyzer {
            tag: TagReg::from_fmt_int(grammemes, "LATN"),
            score: SCORE,
        }
    }
}

impl AnalyzerUnit for LatinAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("LatinAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        if !is_latin(ctx.word_lower()) {
            return;
        }
        let shaped = Shaped::latin(ctx.word_lower());
        let lex = Lex::from_stack(StackSource::from(shaped));
        ctx.push(Parsed::new(lex, self.score));
    }
}
