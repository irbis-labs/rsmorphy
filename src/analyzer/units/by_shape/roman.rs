use crate::{
    analyzer::{units::abc::AnalyzerUnit, MorphAnalyzer},
    container::{abc::*, stack::StackSource, Lex, ParseResult, Parsed, SeenSet, Shaped},
    opencorpora::OpencorporaTagReg,
    shapes::is_roman_number,
};

#[derive(Debug, Clone)]
pub struct RomanAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for RomanAnalyzer {
    fn default() -> Self {
        RomanAnalyzer {
            tag: OpencorporaTagReg::new("ROMN"),
        }
    }
}

impl AnalyzerUnit for RomanAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        _seen_parses: &mut SeenSet,
    ) {
        log::trace!("RomanAnalyzer::parse()");
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        if !is_roman_number(word_lower) {
            return;
        }
        let shaped = Shaped::roman_number(word_lower);
        let score = shaped.score();
        let lex = Lex::from_stack(morph, StackSource::from(shaped));
        result.push(Parsed::new(lex, score));
    }
}
