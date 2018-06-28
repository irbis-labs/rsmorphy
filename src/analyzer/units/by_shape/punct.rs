use analyzer::units::abc::AnalyzerUnit;
use analyzer::MorphAnalyzer;
use container::abc::*;
use container::stack::StackSource;
use container::Lex;
use container::Shaped;
use container::{ParseResult, Parsed, SeenSet};
use opencorpora::OpencorporaTagReg;
use shapes::is_punctuation;

/// This analyzer tags punctuation marks as "PNCT".
///
/// Example: "," -> PNCT

#[derive(Debug, Clone)]
pub struct PunctuationAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for PunctuationAnalyzer {
    fn default() -> Self {
        PunctuationAnalyzer {
            tag: OpencorporaTagReg::new("PNCT"),
        }
    }
}

impl AnalyzerUnit for PunctuationAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        _seen_parses: &mut SeenSet,
    ) {
        trace!("PunctuationAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        if !is_punctuation(word_lower) {
            return;
        }
        let shaped = Shaped::punctuation(word_lower);
        let score = shaped.score();
        let lex = Lex::from_stack(morph, StackSource::from(shaped));
        result.push(Parsed::new(lex, score));
    }
}
