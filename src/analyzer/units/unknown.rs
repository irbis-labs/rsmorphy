use ::analyzer::MorphAnalyzer;
use ::container::{Unknown};
use ::container::{Parsed, ParseResult, SeenSet};
use ::container::{Lex, Score};
use ::container::stack::StackSource;
use ::opencorpora::OpencorporaTagReg;

use super::abc::Analyzer;


#[derive(Debug, Clone)]
pub struct UnknownAnalyzer {
    pub tag: OpencorporaTagReg,
}


impl Default for UnknownAnalyzer {
    fn default() -> Self {
        UnknownAnalyzer {
            tag: OpencorporaTagReg::new("UNKN")
        }
    }
}


impl Analyzer for UnknownAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, seen_parses: &mut SeenSet) {
        trace!("UnknownAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        if seen_parses.is_empty() {
            let container = StackSource::from(Unknown {
                word_lower: word_lower.to_owned(),
            });
            result.push(Parsed {
                score: Score::Real(1.0),
                lex: Lex::from_stack(morph, container),
            });
        }
    }
}

