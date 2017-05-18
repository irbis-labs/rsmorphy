use analyzer::MorphAnalyzer;
use analyzer::units::abc::*;
use container::{Parsed, ParseResult, SeenSet};
use container::{Lex, Score};
use container::abc::*;
use container::stack::StackSource;
use container::HyphenAdverb;
use opencorpora::kind::{Case, Number, PartOfSpeach};
use opencorpora::OpencorporaTagReg;


/// Detect adverbs that starts with "по-".
///
/// Example: по-западному

#[derive(Debug, Clone)]
pub struct HyphenAdverbAnalyzer {
    pub tag: OpencorporaTagReg,
}


impl Default for HyphenAdverbAnalyzer {
    fn default() -> Self {
        HyphenAdverbAnalyzer {
            tag: OpencorporaTagReg::from_str("ADVB")
        }
    }
}



impl HyphenAdverbAnalyzer {
    pub fn should_parse(morph: &MorphAnalyzer, word: &str) -> bool {
        if word.chars().count() < 5 || !word.starts_with("по-") {
            return false;
        }
        morph.parse(&word["по-".len() ..]).into_iter().any(|parsed: Parsed| {
            let tag = parsed.lex.get_tag(morph);
            match (tag.pos, tag.number, tag.case) {
                (Some(PartOfSpeach::Adjf), Some(Number::Sing), Some(Case::Datv)) => true,
                _ => false
            }
        })
    }
}


impl Analyzer for HyphenAdverbAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("HyphenAdverbAnalyzer::parse()");
        trace!(r#" word: "{}", word_lower: "{}" "#, word, word_lower);

        if !Self::should_parse(morph, word_lower) {
            return;
        }

        let container = StackSource::HyphenAdverb(HyphenAdverb {
            word_lower: word_lower.to_string(),
        });
        result.push(Parsed {
            lex: Lex::from_stack(morph, container),
            score: Score::Fake(0.7),
        });
    }
}
