use crate::analyzer::unit::AnalyzerUnit;
use crate::analyzer::MorphAnalyzer;
use crate::container::stack::StackSource;
use crate::container::Lex;
use crate::container::ParseResult;
use crate::container::Parsed;
use crate::container::Score;
use crate::container::SeenSet;
use crate::container::Unknown;
use crate::opencorpora::OpencorporaTagReg;

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
        #[cfg(trace_analyzer)]
        log::trace!("UnknownAnalyzer::parse()");
        #[cfg(trace_analyzer)]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        if seen_parses.is_empty() {
            let lex = Lex::from_stack(morph, StackSource::from(Unknown::new(word_lower)));
            result.list.push(Parsed::new(lex, SCORE));
        }
    }
}
