use inlinable_string::InlinableString;

use crate::analyzer::context::ParseContext;
use crate::analyzer::model::abc::*;
use crate::analyzer::model::stack::StackAffix;
use crate::analyzer::model::Affix;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::unit::AnalyzerUnit;
use crate::analyzer::util::word_splits;

const MIN_REMINDER_LENGTH: usize = 3;
const MAX_PREFIX_LENGTH: usize = 5;
const ESTIMATE_DECAY: f64 = 0.5;

/// Parses the word by parsing only the word suffix
/// (with restrictions on prefix & suffix lengths).
///
/// Example: байткод -> (байт) + код
#[derive(Debug, Clone, Copy)]
pub struct UnknownPrefixAnalyzer {
    estimate_decay: f64,
}

impl UnknownPrefixAnalyzer {
    pub(in crate::analyzer) fn new() -> Self {
        let estimate_decay = ESTIMATE_DECAY;
        UnknownPrefixAnalyzer { estimate_decay }
    }
}

impl AnalyzerUnit for UnknownPrefixAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("UnknownPrefixAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        let morph = ctx.morph();
        let word_lower = InlinableString::from(ctx.word_lower());

        for (prefix, unprefixed_word) in
            word_splits(&word_lower, MIN_REMINDER_LENGTH, MAX_PREFIX_LENGTH)
        {
            let mut subresult = ParseContext::new(morph, unprefixed_word);
            morph.units().dictionary.parse(&mut subresult);

            'iter_parses: for parsed in subresult.finish() {
                let tag_reg = parsed.lex.get_tag(morph);
                if !tag_reg.tags.is_productive() {
                    continue 'iter_parses;
                }
                let stack = parsed.lex.stack.stack.left.stack;
                let affix = Affix::unknown_prefix(prefix);
                let lex = Lex::from_stack(StackAffix::new(stack, affix));
                let score = parsed.score * self.estimate_decay;
                ctx.push_if_not_seen(Parsed::new(lex, score));
            }
        }
    }
}
