use analyzer::MorphAnalyzer;
use analyzer::units::abc::Analyzer;
use container::{Parsed, ParseResult, SeenSet};
use container::{Lex, Score};
use container::Shaped;
use container::ShapeKind;
use container::stack::StackSource;
use opencorpora::OpencorporaTagReg;
use shapes::is_roman_number;


#[derive(Debug, Clone)]
pub struct RomanAnalyzer {
    pub score: f64,
    pub tag: OpencorporaTagReg,
}


impl Default for RomanAnalyzer {
    fn default() -> Self {
        RomanAnalyzer {
            score: 0.9,
            tag: OpencorporaTagReg::new("ROMN")
        }
    }
}


impl Analyzer for RomanAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("RomanAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        if !is_roman_number(word_lower) {
            return;
        }

        let container = Shaped {
            word: word_lower.to_string(),
            kind: ShapeKind::RomanNumber,
        };
        result.push(Parsed {
            lex: Lex::from_stack(morph, StackSource::from(container)),
            score: Score::Fake(self.score),
        });
    }
}
