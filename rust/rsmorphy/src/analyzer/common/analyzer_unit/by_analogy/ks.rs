use std::borrow::Cow;
use std::cmp::Ordering;
use std::iter;
use std::marker::PhantomData;

use lib::collections::dawg_value::PredictionSuffixValue;
use smallvec::SmallVec;
use smart_string::DisplayExt;
use smart_string::SmartString;

use crate::analyzer::common::Affix;
use crate::analyzer::common::DictionaryWord;
use crate::analyzer::common::Main;
use crate::analyzer::common::ParseContext;
use crate::analyzer::common::Seen;
use crate::analyzer::common::WordStruct;
use crate::analyzer::dictionary::PredictionSuffixesDawg;
use crate::analyzer::Dictionary;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;

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
        #[cfg(feature = "log_trace_analyzer")]
        log::trace!("KnownSuffixAnalyzer::parse(); {ctx:?}");

        let morph = ctx.morph();
        let dict = morph.dict();
        let word_lower = ctx.word_lower().clone();

        let word_splits: SmallVec<[usize; 32]> = word_lower
            .chars()
            .map({
                let mut pos = 0;
                move |ch| {
                    pos += ch.len_utf8();
                    pos
                }
            })
            .collect();

        let word_len = word_splits.len();
        if word_len < self.min_word_length {
            return;
        }

        let mut subresult: SmallVec<[(u16, u16, Lex); 16]> = SmallVec::new();

        let mut total_counts: SmallVec<[u32; 4]> = iter::repeat(1)
            .take(dict.paradigm_prefixes().len())
            .collect();

        #[allow(unused_variables)]
        for (prefix_id, prefix, suffixes_dawg) in self.possible_prefixes(dict, &word_lower) {
            #[cfg(feature = "log_trace_analyzer")]
            log::trace!(r#" prefix_id: {}, prefix: "{}" "#, prefix_id, prefix);

            'iter_splits: for suffix_len in dict.prediction_splits(word_len) {
                let split_after_char = word_len - suffix_len;
                debug_assert!(split_after_char > 0);

                let pos = word_splits[split_after_char - 1];

                #[cfg(feature = "log_trace_analyzer")]
                log::trace!("i: {}, pos: {}", suffix_len, pos);

                let (beginning, end) = word_lower.split_at(pos);

                #[cfg(feature = "log_trace_analyzer")]
                log::trace!("word beginning: {}, end: {}", beginning, end);

                let para_data = suffixes_dawg
                    .similar_items(end, dict.char_substitutes())
                    .unwrap();

                for (fixed_suffix, parses) in para_data.into_iter().rev() {
                    let fixed_word = if fixed_suffix == end {
                        Cow::Borrowed(&word_lower)
                    } else {
                        let s: SmartString = format_args!("{beginning}{fixed_suffix}").to_fmt();
                        Cow::Owned(s)
                    };

                    #[cfg(feature = "log_trace_analyzer")]
                    log::trace!(
                        "fixed_suffix: {:?}, fixed_word: {:?}",
                        fixed_suffix,
                        fixed_word
                    );
                    #[cfg(feature = "log_trace_analyzer")]
                    log::trace!("parses: {:?}", parses);

                    'iter_parses: for PredictionSuffixValue {
                        count,
                        para_id,
                        idx,
                    } in parses
                    {
                        let tag_reg = dict.get_tag(para_id, idx);
                        if !dict.is_productive(tag_reg.grammemes) {
                            continue 'iter_parses;
                        }

                        #[cfg(feature = "log_trace_analyzer")]
                        log::trace!("tc: {:?}", total_counts);
                        #[cfg(feature = "log_trace_analyzer")]
                        log::trace!(
                            "prefix_id: {:?}, tc[prefix_id]: {:?}, cnt: {:?}",
                            prefix_id,
                            total_counts[prefix_id],
                            count
                        );

                        total_counts[prefix_id] += count as u32;

                        let seen = Seen::new(fixed_word.as_ref().clone(), tag_reg, Some(para_id));

                        if !ctx.mark_as_seen(seen) {
                            continue 'iter_parses;
                        }

                        let word_lower = WordStruct::unknown(fixed_word.as_ref().clone());
                        let base = DictionaryWord::new(word_lower, para_id, idx);
                        let affix = Affix::known_suffix(fixed_suffix.clone());
                        let lex = Lex::from_main(Main::new(base, Some(affix)));

                        subresult.push((count, prefix_id as u16, lex));
                    }
                    if total_counts[prefix_id] > 1 {
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
        dict: &'m Dictionary,
        word: &'s str,
    ) -> impl Iterator<Item = (usize, &'m str, PredictionSuffixesDawg<'m>)> + 'i {
        dict.paradigm_prefixes()
            .iter()
            .enumerate()
            .rev()
            .filter(move |(_, prefix)| word.starts_with(prefix))
            .map(move |(prefix_idx, prefix)| {
                (
                    prefix_idx,
                    prefix,
                    dict.prediction_suffixes_dawg(prefix_idx),
                )
            })
    }
}

#[cfg(feature = "lang-ru")]
#[cfg(test)]
mod tests_ru {
    use super::*;
    use crate::analyzer::ru::util::test::RU;
    use crate::util::format::ExplainParse;

    #[test]
    fn big_number_of_parses() {
        lib_dev::init_logger();

        let mut buf = RU.make_buffer();
        let mut ctx = buf.prepare("существующой");
        let ks_analyzer = KnownSuffixAnalyzer::new(None, None);

        let tm = std::time::Instant::now();
        ks_analyzer.parse(&mut ctx);
        log::debug!("Elapsed time: {:0.6} s", tm.elapsed().as_secs_f64());

        for (i, parse) in ctx.into_iter().enumerate() {
            log::debug!("{:2}. {}", i + 1, ExplainParse::new(&*RU, &parse));
        }
    }
}
