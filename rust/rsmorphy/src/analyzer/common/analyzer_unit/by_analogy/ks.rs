use std::cmp::Ordering;
use std::iter;
use std::marker::PhantomData;

use inlinable_string::InlinableString;
use smallvec::SmallVec;

use crate::analyzer::common::Affix;
use crate::analyzer::common::DictionaryWord;
use crate::analyzer::common::Main;
use crate::analyzer::common::ParseContext;
use crate::analyzer::common::Seen;
use crate::analyzer::common::WordStruct;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;
use crate::opencorpora::dictionary::PredictionSuffixValue;
use crate::opencorpora::dictionary::PredictionSuffixesDawg;

const MIN_WORD_LENGTH: usize = 4;
const SCORE_MULTIPLIER: f64 = 0.5;

/// Parse the word by checking how the words with similar suffixes
/// are parsed.
///
/// Example: бутявкать -> ...вкать

#[derive(Debug, Clone, Copy)]
pub struct KnownSuffixAnalyzer<L: Lang> {
    min_word_length: usize,
    score_multiplier: f64,
    _lang: PhantomData<L>,
}

impl<L: Lang> KnownSuffixAnalyzer<L> {
    pub fn new(min_word_length: Option<usize>, score_multiplier: Option<f64>) -> Self {
        KnownSuffixAnalyzer {
            min_word_length: min_word_length.unwrap_or(MIN_WORD_LENGTH),
            score_multiplier: score_multiplier.unwrap_or(SCORE_MULTIPLIER),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("KnownSuffixAnalyzer::parse(); {ctx:?}");

        let word_len = ctx.word_lower().chars().count();
        if word_len < self.min_word_length {
            return;
        }

        let morph = ctx.morph();
        let word_lower = InlinableString::from(ctx.word_lower());

        let mut subresult: SmallVec<[(u16, u16, Lex); 16]> = SmallVec::new();

        let mut total_counts: SmallVec<[u32; 4]> = iter::repeat(1)
            .take(morph.dict().paradigm_prefixes.len())
            .collect();

        #[allow(unused_variables)]
        for (prefix_id, prefix, suffixes_dawg) in self.possible_prefixes(morph, &word_lower) {
            #[cfg(feature = "trace_analyzer")]
            log::trace!(r#" prefix_id: {}, prefix: "{}" "#, prefix_id, prefix);

            'iter_splits: for i in morph.dict().prediction_splits(word_len) {
                let pos = word_lower
                    .chars()
                    .take(word_len - i)
                    .map(char::len_utf8)
                    .sum();
                #[cfg(feature = "trace_analyzer")]
                log::trace!("i: {}, pos: {}", i, pos);

                let (word_start, word_end) = word_lower.split_at(pos);
                #[cfg(feature = "trace_analyzer")]
                log::trace!("word_start: {}, word_end: {}", word_start, word_end);

                let para_data = suffixes_dawg
                    .similar_items(word_end, &morph.dict().char_substitutes)
                    .unwrap();
                for (fixed_suffix, parses) in para_data {
                    // TODO use COW here.
                    let fixed_word = if fixed_suffix == word_end {
                        word_lower.clone()
                    } else {
                        InlinableString::from(word_start) + &fixed_suffix
                    };
                    #[cfg(feature = "trace_analyzer")]
                    log::trace!(
                        "fixed_suffix: {:?}, fixed_word: {:?}",
                        fixed_suffix,
                        fixed_word
                    );

                    #[cfg(feature = "trace_analyzer")]
                    log::trace!("parses: {:?}", parses);
                    'iter_parses: for PredictionSuffixValue {
                        count,
                        para_id,
                        idx,
                    } in parses
                    {
                        let tag_reg = morph.dict().get_tag(para_id, idx);
                        // if !tag_reg.tags.is_productive() {
                        if !morph.dict().is_productive(tag_reg.grammemes) {
                            continue 'iter_parses;
                        }

                        #[cfg(feature = "trace_analyzer")]
                        log::trace!("tc: {:?}", total_counts);
                        #[cfg(feature = "trace_analyzer")]
                        log::trace!(
                            "prefix_id: {:?}, tc[prefix_id]: {:?}, cnt: {:?}",
                            prefix_id,
                            total_counts[prefix_id as usize],
                            count
                        );
                        total_counts[prefix_id as usize] += count as u32;

                        let seen = Seen::new(fixed_word.clone(), tag_reg, Some(para_id));

                        if !ctx.mark_as_seen(seen) {
                            continue 'iter_parses;
                        }

                        let word_lower = WordStruct::unknown(fixed_word.clone());
                        let base = DictionaryWord::new(word_lower, para_id, idx);
                        let affix = Affix::known_suffix(fixed_suffix.clone());
                        let lex = Lex::from_main(Main::new(base, Some(affix)));

                        subresult.push((count, prefix_id, lex));
                    }
                    if total_counts[prefix_id as usize] > 1 {
                        break 'iter_splits;
                    }
                }
            }
        }

        let mut subresult: SmallVec<[Parse; 16]> = subresult
            .into_iter()
            .map(|(cnt, prefix_id, lex)| {
                let score = Score::Fake(
                    self.score_multiplier * f64::from(cnt)
                        / f64::from(total_counts[prefix_id as usize]),
                );
                Parse { lex, score }
            })
            .collect();

        subresult.sort_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(Ordering::Equal)
                .reverse()
        });
        ctx.extend(subresult);
    }

    fn possible_prefixes<'m: 'i, 's: 'i, 'i>(
        &self,
        morph: &'m L,
        word: &'s str,
    ) -> impl Iterator<Item = (u16, &'m str, &'m PredictionSuffixesDawg)> + 'i {
        use std::ops::Deref;
        morph
            .dict()
            .paradigm_prefixes_rev
            .iter()
            .filter(move |(_, prefix)| word.starts_with(prefix.deref()))
            .map(move |(prefix_idx, prefix)| {
                (
                    *prefix_idx,
                    prefix.deref(),
                    &morph.dict().prediction_suffixes_dawgs[*prefix_idx as usize],
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::ru::util::test::RU;
    use crate::util::format::ExplainParse;
    use crate::util::test::init_logger;

    #[test]
    fn big_count() {
        init_logger();

        let mut ctx = ParseContext::new(&*RU, "существующой");
        let ks_analyzer = KnownSuffixAnalyzer::new(None, None);

        let tm = std::time::Instant::now();
        ks_analyzer.parse(&mut ctx);
        log::debug!("Elapsed time: {:0.6} s", tm.elapsed().as_secs_f64());

        let result = ctx.finish();
        for (i, parse) in result.into_iter().enumerate() {
            log::debug!("{:2}. {}", i + 1, ExplainParse::new(&*RU, &parse));
        }
    }
}
