use std::marker::PhantomData;

use lib::collections::gram_registry::GramRegistry;
use lib::types::tag::TagReg;

use crate::analyzer::common::ParseContext;
use crate::analyzer::common::Shaped;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;
use crate::shapes::is_latin;

const SCORE: Score = Score::Fake(0.9);

/// This analyzer marks latin words with "LATN" tag.
///
/// Example: "pdf" -> LATN

#[derive(Debug)]
pub struct LatinAnalyzer<L: Lang> {
    pub tag: TagReg,
    pub score: Score,
    _lang: PhantomData<L>,
}

impl<L: Lang> LatinAnalyzer<L> {
    pub fn new(grammemes: GramRegistry<'_>, score: Option<Score>) -> Self {
        LatinAnalyzer {
            tag: grammemes.parse_tag("LATN"),
            score: score.unwrap_or(SCORE),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "log_trace_analyzer")]
        log::trace!("LatinAnalyzer::parse(); {ctx:?}");

        if !is_latin(ctx.word_lower()) {
            return;
        }
        let lex = Lex::from_base(Shaped::latin(ctx.word_lower().clone()));
        ctx.push(Parse::new(lex, self.score));
    }
}
