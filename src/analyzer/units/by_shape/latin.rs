use analyzer::MorphAnalyzer;
use analyzer::units::abc::Analyzer;
use container::{Parsed, ParseResult, SeenSet};
use container::{Lex, Score};
use container::Shaped;
use container::ShapeKind;
use container::stack::StackSource;
use opencorpora::OpencorporaTagReg;
use shapes::is_latin;


/// This analyzer marks latin words with "LATN" tag.
///
/// Example: "pdf" -> LATN

#[derive(Debug, Clone)]
pub struct LatinAnalyzer {
    pub score: f64,
    pub tag: OpencorporaTagReg,
}


impl Default for LatinAnalyzer {
    fn default() -> Self {
        LatinAnalyzer {
            score: 0.9,
            tag: OpencorporaTagReg::from_str("LATN")
        }
    }
}


impl Analyzer for LatinAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("LatinAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        if !is_latin(word_lower) {
            return;
        }

        let container = Shaped {
            word: word_lower.to_string(),
            kind: ShapeKind::Latin,
        };
        result.push(Parsed {
            lex: Lex::from_stack(morph, StackSource::from(container)),
            score: Score::Fake(self.score),
        });
    }
}
