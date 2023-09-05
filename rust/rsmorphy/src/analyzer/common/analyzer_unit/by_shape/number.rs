use std::marker::PhantomData;
use std::str::FromStr;

use crate::analyzer::common::ParseContext;
use crate::analyzer::common::Shaped;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;
use crate::model::gram::GramRegistry;
use crate::model::tag::TagReg;

const SCORE: Score = Score::Fake(0.9);

/// This analyzer marks integer numbers with "NUMB,intg" or "NUMB,real" tags.
///
/// Example: "42" -> NUMB,intg; "3.14" -> NUMB,real
///
/// .. note::
///
/// Don't confuse it with "NUMR": "тридцать" -> NUMR

#[derive(Debug)]
pub struct NumberAnalyzer<L: Lang> {
    pub tag_int: TagReg,
    pub tag_real: TagReg,
    pub score: Score,
    _lang: PhantomData<L>,
}

impl<L: Lang> NumberAnalyzer<L> {
    pub fn new(grammemes: GramRegistry<'_>, score: Option<Score>) -> Self {
        NumberAnalyzer {
            tag_int: TagReg::from_fmt_int(grammemes, "NUMB,intg"),
            tag_real: TagReg::from_fmt_int(grammemes, "NUMB,real"),
            score: score.unwrap_or(SCORE),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("NumberAnalyzer::parse(); {ctx:?}");

        let word_lower = ctx.word_lower();

        // TODO Improve number parser [#12]
        let shaped = match () {
            _ if i128::from_str(word_lower).is_ok() => Shaped::integer(word_lower.clone()),
            _ if f64::from_str(word_lower).is_ok() => Shaped::fractional(word_lower.clone()),
            _ => return,
        };
        let lex = Lex::from_base(shaped);
        ctx.push(Parse::new(lex, self.score));
    }
}
