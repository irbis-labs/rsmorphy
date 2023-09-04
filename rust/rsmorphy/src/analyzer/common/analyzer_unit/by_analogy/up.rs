use std::marker::PhantomData;

use crate::analyzer::common::Affix;
use crate::analyzer::common::DictionaryAnalyzer;
use crate::analyzer::common::ParseContext;
use crate::analyzer::Lang;
use crate::analyzer::Parse;
use crate::Score;

const MIN_REMINDER_LENGTH: usize = 3;
const MAX_PREFIX_LENGTH: usize = 5;
const SCORE_MULTIPLIER: f64 = 0.5;

/// Parses the word by parsing only the word suffix
/// (with restrictions on prefix & suffix lengths).
///
/// Example: байткод -> (байт) + код
#[derive(Debug, Clone, Copy)]
pub struct UnknownPrefixAnalyzer<L: Lang> {
    dictionary: DictionaryAnalyzer<L>,
    min_reminder_length: usize,
    max_prefix_length: usize,
    score_multiplier: f64,
    _lang: PhantomData<L>,
}

impl<L: Lang> UnknownPrefixAnalyzer<L> {
    pub fn new(
        dictionary: Option<DictionaryAnalyzer<L>>,
        min_reminder_length: Option<usize>,
        max_prefix_length: Option<usize>,
        score_multiplier: Option<f64>,
    ) -> Self {
        UnknownPrefixAnalyzer {
            dictionary: dictionary.unwrap_or_else(|| DictionaryAnalyzer::new(None)),
            min_reminder_length: min_reminder_length.unwrap_or(MIN_REMINDER_LENGTH),
            max_prefix_length: max_prefix_length.unwrap_or(MAX_PREFIX_LENGTH),
            score_multiplier: score_multiplier.unwrap_or(SCORE_MULTIPLIER),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("UnknownPrefixAnalyzer::parse(); {ctx:?}");

        let morph = ctx.morph();
        let word_lower = ctx.word_lower().clone();
        let mut parse_buffer = morph.make_buffer();

        for (prefix, unprefixed_word) in word_splits(
            &word_lower,
            self.min_reminder_length,
            self.max_prefix_length,
        ) {
            let mut subresult = parse_buffer.prepare(unprefixed_word);
            self.dictionary.parse(&mut subresult);

            'iter_parses: for Parse { mut lex, score } in subresult.estimate().drain() {
                let tag_reg = morph.get_tag(&lex);
                if !morph.dict().is_productive(tag_reg.grammemes) {
                    continue 'iter_parses;
                }

                debug_assert!(lex.right.is_none() && lex.particle.is_none());

                lex.left.affix = Some(Affix::unknown_prefix(prefix));
                let score = Score::Fake(score.value() * self.score_multiplier);

                ctx.push_if_not_seen(Parse::new(lex, score));
            }
        }
    }
}

/// Returns all splits of a `word` (taking into account `min_reminder` and `max_prefix_length`).
pub fn word_splits<'w: 'i, 'i>(
    word: &'w str,
    min_reminder: usize,
    max_prefix_length: usize,
) -> impl Iterator<Item = (&'w str, &'w str)> + 'i {
    let word_len = word.chars().count();
    let max_split = word_len.saturating_sub(min_reminder).min(max_prefix_length);

    #[cfg(feature = "trace_analyzer")]
    log::trace!("max_split: {}", max_split);

    word.chars().take(max_split).map({
        let mut pos = 0;
        move |ch| {
            pos += ch.len_utf8();
            word.split_at(pos)
        }
    })
}
