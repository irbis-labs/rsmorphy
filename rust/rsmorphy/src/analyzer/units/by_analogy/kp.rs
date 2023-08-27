use crate::analyzer::context::ParseContext;
use crate::analyzer::model::abc::*;
use crate::analyzer::model::stack::StackAffix;
use crate::analyzer::model::stack::StackHyphenated;
use crate::analyzer::model::stack::StackParticle;
use crate::analyzer::model::Affix;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::unit::AnalyzerUnit;
use crate::analyzer::MorphAnalyzer;

const MIN_REMINDER_LENGTH: usize = 3;
const ESTIMATE_DECAY: f64 = 0.75;

/// Parse the word by checking if it starts with a known prefix
/// and parsing the reminder.
///
/// Example: псевдокошка -> (псевдо) + кошка.

#[derive(Debug, Clone, Copy)]
pub struct KnownPrefixAnalyzer {
    min_reminder_length: usize,
    estimate_decay: f64,
}

impl KnownPrefixAnalyzer {
    pub(in crate::analyzer) fn new() -> Self {
        KnownPrefixAnalyzer {
            min_reminder_length: MIN_REMINDER_LENGTH,
            estimate_decay: ESTIMATE_DECAY,
        }
    }
}

impl AnalyzerUnit for KnownPrefixAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("KnownPrefixAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        let word = ctx.word();
        let word_len = word.chars().count();
        if word_len <= self.min_reminder_length {
            // This analyzer works only for longer words.
            return;
        }
        let limit = word_len - self.min_reminder_length;
        let morph = ctx.morph();

        #[cfg(feature = "trace_analyzer")]
        log::trace!("possible_splits for word = {word}, limit = {limit}");
        for (prefix, unprefixed_word) in self.possible_splits(morph, word, limit) {
            #[cfg(feature = "trace_analyzer")]
            log::trace!("prefix = {prefix:?}, unprefixed_word = {unprefixed_word:?}");
            'iter_parses: for parsed in morph.parse(unprefixed_word) {
                let tag_reg = parsed.lex.get_tag(morph);
                if !tag_reg.tags.is_productive() {
                    continue 'iter_parses;
                }
                let StackParticle {
                    stack: StackHyphenated { left, right },
                    particle: hsp,
                } = parsed.lex.stack;
                if hsp.is_some() || right.is_some() || left.affix().is_some() {
                    #[cfg(feature = "trace_analyzer")]
                    log::trace!(
                        "skipping:  hsp.is_some = {},  right.is_some = {} || left.affix.is_some = {}",
                        hsp.is_some(),
                        right.is_some(),
                        left.affix().is_some()
                    );
                    continue;
                }
                let (stack_source, _left_affix) = left.into_parts();
                let affix = Some(Affix::known_prefix(prefix));
                let stack = StackAffix::new(stack_source, affix);
                let lex = Lex::from_stack(stack);
                let score = parsed.score * self.estimate_decay;
                ctx.push_if_not_seen(Parsed::new(lex, score));
            }
        }
    }
}

impl KnownPrefixAnalyzer {
    fn possible_splits<'m: 'i, 's: 'i, 'i>(
        &self,
        morph: &'m MorphAnalyzer,
        word: &'s str,
        limit: usize,
    ) -> impl Iterator<Item = (&'s str, &'s str)> + 'i {
        let word_prefixes = morph.dict().prediction_prefixes.sorted_prefixes(word);
        #[cfg(feature = "trace_analyzer")]
        log::trace!("word_prefixes: {}", word_prefixes.join(", "));
        word_prefixes
            .into_iter()
            .map(move |prefix| (prefix.chars().count(), prefix))
            .filter(move |&(char_len, _prefix)| char_len <= limit)
            .map(move |(char_len, prefix)| {
                let pos = word.chars().take(char_len).map(char::len_utf8).sum();
                (prefix, &word[pos..])
                // FIXME why not this?
                //                (prefix, &word[prefix.len() ..])
            })
    }
}
