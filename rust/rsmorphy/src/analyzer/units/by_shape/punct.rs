use crate::analyzer::context::ParseContext;
use crate::analyzer::model::abc::*;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Shaped;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::tag::TagReg;
use crate::shapes::is_punctuation;

/// This analyzer tags punctuation marks as "PNCT".
///
/// Example: "," -> PNCT

#[derive(Debug, Clone)]
pub struct PunctuationAnalyzer {
    pub tag: TagReg,
}

impl PunctuationAnalyzer {
    pub(in crate::analyzer) fn new(grammemes: &GramRegistry) -> Self {
        let tag = TagReg::from_fmt_int(grammemes, "PNCT");
        PunctuationAnalyzer { tag }
    }
}

impl AnalyzerUnit for PunctuationAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("PunctuationAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        let word_lower = ctx.word_lower();
        if !is_punctuation(word_lower) {
            return;
        }
        let shaped = Shaped::punctuation(word_lower);
        let score = shaped.score();
        let lex = Lex::from_stack(StackSource::from(shaped));
        ctx.push(Parsed::new(lex, score));
    }
}
