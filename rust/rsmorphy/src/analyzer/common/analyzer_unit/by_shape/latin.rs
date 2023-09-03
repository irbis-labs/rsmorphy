use std::marker::PhantomData;

use crate::analyzer::common::ParseContext;
use crate::analyzer::common::Shaped;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::tag::TagReg;
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
    pub fn new(grammemes: &GramRegistry, score: Option<Score>) -> Self {
        LatinAnalyzer {
            tag: TagReg::from_fmt_int(grammemes, "LATN"),
            score: score.unwrap_or(SCORE),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("LatinAnalyzer::parse(); {ctx:?}");

        if !is_latin(ctx.word_lower()) {
            return;
        }
        let lex = Lex::from_base(Shaped::latin(ctx.word_lower()));
        ctx.push(Parse::new(lex, self.score));
    }
}
