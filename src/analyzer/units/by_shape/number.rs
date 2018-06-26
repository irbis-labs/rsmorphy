use std::str::FromStr;

use analyzer::MorphAnalyzer;
use analyzer::units::abc::AnalyzerUnit;
use container::{Parsed, ParseResult, SeenSet};
use container::Lex;
use container::abc::*;
use container::Shaped;
use container::stack::StackSource;
use opencorpora::OpencorporaTagReg;


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
            tag_int: OpencorporaTagReg::new("NUMB,intg"),
            tag_real: OpencorporaTagReg::new("NUMB,real"),
        }
    }
}


impl AnalyzerUnit for NumberAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("NumberAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

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
        result.push(Parsed::new(lex, score));
    }
}
