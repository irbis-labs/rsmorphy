use crate::{
    analyzer::{units::abc::AnalyzerUnit, MorphAnalyzer},
    container::{abc::*, stack::StackSource, Lex, ParseResult, Parsed, SeenSet, Shaped},
    opencorpora::OpencorporaTagReg,
    shapes::is_punctuation,
};

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
            tag: OpencorporaTagReg::from_fmt_int("PNCT"),
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
        log::trace!("PunctuationAnalyzer::parse()");
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        if !is_punctuation(word_lower) {
            return;
        }
        let shaped = Shaped::punctuation(word_lower);
        let score = shaped.score();
        let lex = Lex::from_stack(morph, StackSource::from(shaped));
        result.push(Parsed::new(lex, score));
    }
}
