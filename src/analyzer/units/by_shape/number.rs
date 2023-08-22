use std::str::FromStr;

use crate::analyzer::units::abc::AnalyzerUnit;
use crate::analyzer::MorphAnalyzer;
use crate::container::abc::*;
use crate::container::stack::StackSource;
use crate::container::Lex;
use crate::container::ParseResult;
use crate::container::Parsed;
use crate::container::SeenSet;
use crate::container::Shaped;
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
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        _seen_parses: &mut SeenSet,
    ) {
        log::trace!("NumberAnalyzer::parse()");
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        // TODO Improve number parser [#12]
        let shaped = if i128::from_str(word_lower).is_ok() {
            Shaped::number(word_lower, false)
        } else if f64::from_str(word_lower).is_ok() {
            Shaped::number(word_lower, true)
        } else {
            return;
        };
        let score = shaped.score();
        let lex = Lex::from_stack(morph, StackSource::from(shaped));
        result.list.push(Parsed::new(lex, score));
    }
}
