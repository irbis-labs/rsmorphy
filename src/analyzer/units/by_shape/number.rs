use std::str::FromStr;

use crate::analyzer::context::ParseContext;
use crate::analyzer::model::abc::*;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Shaped;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::OpencorporaTagReg;

/// This analyzer marks integer numbers with "NUMB,int" or "NUMB,real" tags.
///
/// Example: "42" -> NUMB,intg; "3.14" -> NUMB,real
///
/// .. note::
///
/// Don't confuse it with "NUMR": "тридцать" -> NUMR

#[derive(Debug, Clone)]
pub struct NumberAnalyzer {
    pub tag_int: OpencorporaTagReg,
    pub tag_real: OpencorporaTagReg,
}

impl Default for NumberAnalyzer {
    fn default() -> Self {
        NumberAnalyzer {
            tag_int: OpencorporaTagReg::from_fmt_int("NUMB,intg"),
            tag_real: OpencorporaTagReg::from_fmt_int("NUMB,real"),
        }
    }
}

impl AnalyzerUnit for NumberAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(trace_analyzer)]
        log::trace!("NumberAnalyzer::parse()");
        #[cfg(trace_analyzer)]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        let word_lower = ctx.word_lower();

        // TODO Improve number parser [#12]
        let shaped = if i128::from_str(word_lower).is_ok() {
            Shaped::number(word_lower, false)
        } else if f64::from_str(word_lower).is_ok() {
            Shaped::number(word_lower, true)
        } else {
            return;
        };
        let score = shaped.score();
        let lex = Lex::from_stack(StackSource::from(shaped));
        ctx.push(Parsed::new(lex, score));
    }
}
