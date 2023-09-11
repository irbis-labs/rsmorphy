use std::marker::PhantomData;

use crate::analyzer::common::Affix;
use crate::analyzer::common::ParseContext;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::data_struct::prefixes::Prefixes;

const MIN_REMINDER_LENGTH: usize = 3;
const SCORE_MULTIPLIER: f64 = 0.75;

/// Parse the word by checking if it starts with a known prefix
/// and parsing the reminder.
///
/// Example: псевдокошка -> (псевдо) + кошка.

#[derive(Debug)]
pub struct KnownPrefixAnalyzer<L: Lang> {
    known_prefixes: Prefixes,
    min_reminder_length: usize,
    score_multiplier: f64,
    _lang: PhantomData<L>,
}

impl<L: Lang> KnownPrefixAnalyzer<L> {
    pub fn new(
        known_prefixes: Prefixes,
        min_reminder_length: Option<usize>,
        score_multiplier: Option<f64>,
    ) -> Self {
        KnownPrefixAnalyzer {
            known_prefixes,
            min_reminder_length: min_reminder_length.unwrap_or(MIN_REMINDER_LENGTH),
            score_multiplier: score_multiplier.unwrap_or(SCORE_MULTIPLIER),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "log_trace_analyzer")]
        log::trace!("KnownPrefixAnalyzer::parse(); {ctx:?}");

        let word = ctx.word();
        let word_len = word.chars().count();
        if word_len <= self.min_reminder_length {
            // This analyzer works only for longer words.
            return;
        }
        let limit = word_len - self.min_reminder_length;
        let morph = ctx.morph();

        #[cfg(feature = "log_trace_analyzer")]
        log::trace!("possible_splits for word = {word}, limit = {limit}");

        let max_len = self.known_prefixes.max_len.min(limit);
        let min_len = self.known_prefixes.min_len.max(1);

        let mut parse_buffer = morph.make_buffer();

        // possible splits
        for prefix_len in (min_len..=max_len).rev() {
            let pos = word.chars().take(prefix_len).map(char::len_utf8).sum();
            let (prefix, unprefixed_word) = word.split_at(pos);

            #[cfg(feature = "log_trace_analyzer")]
            log::trace!("prefix = {prefix:?}, unprefixed_word = {unprefixed_word:?}");

            if !self.known_prefixes.contains_hinted(prefix, prefix_len) {
                #[cfg(feature = "log_trace_analyzer")]
                log::trace!("skipping: prefix not found in known_prefixes");
                continue;
            }

            'iter_parses: for parse in parse_buffer
                .prepare(unprefixed_word)
                .parse()
                .estimate()
                .drain()
            {
                #[cfg(feature = "log_trace_analyzer")]
                log::trace!("parse = {:?}", parse);

                let Parse { lex, score } = parse;

                let tag_reg = morph.get_tag(&lex);
                if !morph.dict().is_productive(tag_reg.grammemes) {
                    continue 'iter_parses;
                }

                let Lex {
                    mut left,
                    right,
                    particle,
                } = lex;

                if particle.is_some() || right.is_some() || left.affix.is_some() {
                    #[cfg(feature = "log_trace_analyzer")]
                    log::trace!(
                        "skipping:  particle.is_some = {}  or  right.is_some = {}  or  left.affix.is_some = {}",
                        particle.is_some(),
                        right.is_some(),
                        left.affix.is_some()
                    );
                    continue;
                }

                left.affix = Some(Affix::known_prefix(prefix));
                let lex = Lex::new(left, right, particle);
                let score = score * self.score_multiplier;

                ctx.push_if_not_seen(Parse::new(lex, score));
            }
        }
    }
}
