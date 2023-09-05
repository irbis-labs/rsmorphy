use std::marker::PhantomData;

use crate::analyzer::common::ParseContext;
use crate::analyzer::common::Shaped;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;
use crate::model::gram::GramRegistry;
use crate::model::tag::TagReg;
use crate::shapes::is_punctuation;

const SCORE: Score = Score::Fake(0.9);

/// This analyzer tags punctuation marks as "PNCT".
///
/// Example: "," -> PNCT

#[derive(Debug, Clone)]
pub struct PunctuationAnalyzer<L: Lang> {
    pub tag: TagReg,
    pub score: Score,
    _lang: PhantomData<L>,
}

impl<L: Lang> PunctuationAnalyzer<L> {
    pub fn new(grammemes: GramRegistry<'_>, score: Option<Score>) -> Self {
        PunctuationAnalyzer {
            tag: TagReg::from_fmt_int(grammemes, "PNCT"),
            score: score.unwrap_or(SCORE),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("PunctuationAnalyzer::parse(); {ctx:?}");

        let word_lower = ctx.word_lower();
        if !is_punctuation(word_lower) {
            return;
        }
        let lex = Lex::from_base(Shaped::punctuation(word_lower.clone()));
        ctx.push(Parse::new(lex, self.score));
    }
}
