use std::marker::PhantomData;

use lib::collections::gram_registry::GramRegistry;
use lib::types::tag::TagReg;

use crate::analyzer::common::parse::Parse;
use crate::analyzer::common::Lex;
use crate::analyzer::common::ParseContext;
use crate::analyzer::common::Score;
use crate::analyzer::common::Unknown;
use crate::analyzer::Lang;

const SCORE: Score = Score::Fake(1.0);

#[derive(Debug, Clone)]
pub struct UnknownAnalyzer<L: Lang> {
    pub tag: TagReg,
    score: Score,
    _lang: PhantomData<L>,
}

impl<L: Lang> UnknownAnalyzer<L> {
    pub fn new(grammemes: GramRegistry<'_>, score: Option<Score>) -> Self {
        UnknownAnalyzer {
            tag: grammemes.parse_tag("UNKN"),
            score: score.unwrap_or(SCORE),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "log_trace_analyzer")]
        log::trace!("UnknownAnalyzer::parse(); {ctx:?}");

        if ctx.seen_parses().is_empty() {
            let lex = Lex::from_base(Unknown::new(ctx.word_lower().clone()));
            ctx.push(Parse::new(lex, self.score));
        }
    }
}
