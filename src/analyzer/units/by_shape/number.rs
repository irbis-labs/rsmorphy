use std::str::FromStr;

use analyzer::MorphAnalyzer;
use analyzer::units::abc::Analyzer;
use container::{Parsed, ParseResult, SeenSet};
use container::{Lex, Score};
use container::Shaped;
use container::ShapeKind;
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
    pub score: f64,
    pub tag_int: OpencorporaTagReg,
    pub tag_real: OpencorporaTagReg,
}


impl Default for NumberAnalyzer {
    fn default() -> Self {
        NumberAnalyzer {
            score: 0.9,
            tag_int: OpencorporaTagReg::from_str("NUMB,intg"),
            tag_real: OpencorporaTagReg::from_str("NUMB,real"),
        }
    }
}


impl Analyzer for NumberAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("NumberAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        // TODO "," => "."
        let kind = if u64::from_str(word_lower).is_ok() {
            ShapeKind::Number { is_float: false }
        } else {
            if f64::from_str(word_lower).is_ok() {
                ShapeKind::Number { is_float: true }
            } else {
                return;
            }
        };

        let container = Shaped {
            word: word_lower.to_string(),
            kind: kind,
        };
        result.push(Parsed {
            lex: Lex::from_stack(morph, StackSource::from(container)),
            score: Score::Fake(self.score),
        });
    }
}
