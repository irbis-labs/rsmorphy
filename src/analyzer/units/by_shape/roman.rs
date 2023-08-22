use crate::analyzer::unit::AnalyzerUnit;
use crate::analyzer::MorphAnalyzer;
use crate::container::abc::*;
use crate::container::stack::StackSource;
use crate::container::Lex;
use crate::container::ParseResult;
use crate::container::Parsed;
use crate::container::SeenSet;
use crate::container::Shaped;
use crate::opencorpora::OpencorporaTagReg;
use crate::shapes::is_roman_number;

#[derive(Debug, Clone)]
pub struct RomanAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for RomanAnalyzer {
    fn default() -> Self {
        RomanAnalyzer {
            tag: OpencorporaTagReg::from_fmt_int("ROMN"),
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
        #[cfg(trace_analyzer)]
        log::trace!("RomanAnalyzer::parse()");
        #[cfg(trace_analyzer)]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);
        if !is_roman_number(word_lower) {
            return;
        }
        let shaped = Shaped::roman_number(word_lower);
        let score = shaped.score();
        let lex = Lex::from_stack(morph, StackSource::from(shaped));
        result.list.push(Parsed::new(lex, score));
    }
}
