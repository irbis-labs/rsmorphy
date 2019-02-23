use crate::{
    analyzer::{units::abc::AnalyzerUnit, MorphAnalyzer},
    container::{stack::StackSource, Lex, ParseResult, Parsed, Score, SeenSet, Unknown},
    opencorpora::OpencorporaTagReg,
};

const SCORE: Score = Score::Fake(1.0);

#[derive(Debug, Clone)]
pub struct UnknownAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for UnknownAnalyzer {
    fn default() -> Self {
        UnknownAnalyzer {
            tag: OpencorporaTagReg::from_fmt_int("UNKN"),
        }
    }
}

impl AnalyzerUnit for UnknownAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        seen_parses: &mut SeenSet,
    ) {
        log::trace!("UnknownAnalyzer::parse()");
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        if seen_parses.is_empty() {
            let lex = Lex::from_stack(morph, StackSource::from(Unknown::new(word_lower)));
            result.push(Parsed::new(lex, SCORE));
        }
    }
}
