use ::analyzer::MorphAnalyzer;
use ::container::{Unknown};
use ::container::{Parsed, ParseResult, SeenSet};
use ::container::{Lex, Score};
use ::container::stack::StackSource;
use ::opencorpora::OpencorporaTagReg;

use super::abc::AnalyzerUnit;


const SCORE: Score = Score::Fake(1.0);


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

impl AnalyzerUnit for UnknownAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, seen_parses: &mut SeenSet) {
        trace!("UnknownAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        if seen_parses.is_empty() {
            let lex = Lex::from_stack(morph, StackSource::from(Unknown::new(word_lower)));
            result.push(Parsed::new(lex, SCORE));
        }
    }
}
