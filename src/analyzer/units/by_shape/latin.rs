use crate::{
    analyzer::{units::abc::AnalyzerUnit, MorphAnalyzer},
    container::{abc::*, stack::StackSource, Lex, ParseResult, Parsed, SeenSet, Shaped},
    opencorpora::OpencorporaTagReg,
    shapes::is_latin,
};

/// This analyzer marks latin words with "LATN" tag.
///
/// Example: "pdf" -> LATN

#[derive(Debug, Clone)]
pub struct LatinAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for LatinAnalyzer {
    fn default() -> Self {
        LatinAnalyzer {
            tag: OpencorporaTagReg::from_fmt_int("LATN"),
        }
    }
}

impl AnalyzerUnit for LatinAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        _seen_parses: &mut SeenSet,
    ) {
        log::trace!("LatinAnalyzer::parse()");
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        if !is_latin(word_lower) {
            return;
        }
        let shaped = Shaped::latin(word_lower);
        let score = shaped.score();
        let lex = Lex::from_stack(morph, StackSource::from(shaped));
        result.push(Parsed::new(lex, score));
    }
}
