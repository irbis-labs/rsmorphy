use analyzer::MorphAnalyzer;
use analyzer::units::abc::Analyzer;
use container::{Parsed, ParseResult, SeenSet};
use container::{Lex, Score};
use container::Shaped;
use container::ShapeKind;
use container::stack::StackSource;
use opencorpora::OpencorporaTagReg;
use shapes::is_punctuation;


/// This analyzer tags punctuation marks as "PNCT".
///
/// Example: "," -> PNCT

#[derive(Debug, Clone)]
pub struct PunctuationAnalyzer {
    pub score: f64,
    pub tag: OpencorporaTagReg,
}


impl Default for PunctuationAnalyzer {
    fn default() -> Self {
        PunctuationAnalyzer {
            score: 0.9,
            tag: OpencorporaTagReg::new("PNCT")
        }
    }
}


impl Analyzer for PunctuationAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("PunctuationAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        if !is_punctuation(word_lower) {
            return;
        }

        let container = Shaped {
            word: word_lower.to_string(),
            kind: ShapeKind::Punctuation,
        };
        result.push(Parsed {
            lex: Lex::from_stack(morph, StackSource::from(container)),
            score: Score::Fake(self.score),
        });
    }
}
