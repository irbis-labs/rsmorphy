use inlinable_string::InlinableString;

use crate::analyzer::unit::AnalyzerUnit;
use crate::analyzer::MorphAnalyzer;
use crate::container::stack::StackAffix;
use crate::container::Affix;
use crate::container::Dictionary;
use crate::container::Lex;
use crate::container::ParseResult;
use crate::container::Parsed;
use crate::container::Score;
use crate::container::Seen;
use crate::container::SeenSet;
use crate::container::WordStruct;
use crate::opencorpora::dictionary::PredictionSuffixesDawg;
use crate::opencorpora::dictionary::HHH;

/// Parse the word by checking how the words with similar suffixes
/// are parsed.
///
/// Example: бутявкать -> ...вкать

#[derive(Debug, Clone, Copy)]
pub struct KnownSuffixAnalyzer {
    pub min_word_length: usize,
    pub estimate_decay: f64,
}

impl Default for KnownSuffixAnalyzer {
    fn default() -> Self {
        KnownSuffixAnalyzer {
            min_word_length: 4,
            estimate_decay: 0.5,
        }
    }
}

impl AnalyzerUnit for KnownSuffixAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        seen_parses: &mut SeenSet,
    ) {
        #[cfg(trace_analyzer)]
        log::trace!("KnownSuffixAnalyzer::parse()");
        #[cfg(trace_analyzer)]
        log::trace!(r#" word: "{}", word_lower: "{}" "#, word, word_lower);

        let char_len: usize = word_lower.chars().count();

        if char_len < self.min_word_length {
            return;
        }

        let mut subresult: Vec<(u16, u16, Lex)> = Vec::new();

        let mut total_counts: Vec<u32> = vec![1; morph.dict().paradigm_prefixes.len()];

        for (prefix_id, prefix, suffixes_dawg) in self.possible_prefixes(morph, word_lower) {
            #[cfg(trace_analyzer)]
            log::trace!(r#" prefix_id: {}, prefix: "{}" "#, prefix_id, prefix);

            'iter_splits: for i in morph.dict().prediction_splits(char_len - 1) {
                let pos = word_lower
                    .chars()
                    .take(char_len - i)
                    .map(char::len_utf8)
                    .sum();
                #[cfg(trace_analyzer)]
                log::trace!("i: {}, pos: {}", i, pos);

                let (word_start, word_end) = (&word_lower[..pos], &word_lower[pos..]);
                #[cfg(trace_analyzer)]
                log::trace!("word_start: {}, word_end: {}", word_start, word_end);

                let para_data = suffixes_dawg.similar_items(word_end, &morph.dict().char_substitutes);
                for (fixed_suffix, parses) in para_data {
                    let fixed_word = if fixed_suffix == word_end {
                        word_lower.into()
                    } else {
                        InlinableString::from(word_start) + &fixed_suffix
                    };
                    #[cfg(trace_analyzer)]
                    log::trace!(
                        "fixed_suffix: {:?}, fixed_word: {:?}",
                        fixed_suffix,
                        fixed_word
                    );

                    #[cfg(trace_analyzer)]
                    log::trace!("parses: {:?}", parses);
                    'iter_parses: for HHH(cnt, para_id, idx) in parses {
                        let tag = morph.dict().get_tag(para_id.into(), idx.into());

                        if !tag.is_productive() {
                            continue 'iter_parses;
                        }

                        #[cfg(trace_analyzer)]
                        log::trace!("tc: {:?}", total_counts);
                        #[cfg(trace_analyzer)]
                        log::trace!(
                            "prefix_id: {:?}, tc[prefix_id]: {:?}, cnt: {:?}",
                            prefix_id,
                            total_counts[prefix_id as usize],
                            cnt
                        );
                        total_counts[prefix_id as usize] += cnt as u32;

                        let seen = Seen::new(fixed_word.clone(), tag, para_id);

                        if !seen_parses.insert(&seen) {
                            continue 'iter_parses;
                        }

                        let word_lower = WordStruct::unknown(seen.word);
                        let source = Dictionary::new(word_lower, para_id, idx);
                        let affix = Affix::known_suffix(fixed_suffix.clone());
                        let container = StackAffix::new(source, affix);

                        subresult.push((cnt, prefix_id, Lex::from_stack(morph, container)));
                    }
                    if total_counts[prefix_id as usize] > 1 {
                        break 'iter_splits;
                    }
                }
            }
        }

        let mut subresult: Vec<Parsed> = subresult
            .into_iter()
            .map(|(cnt, prefix_id, lex)| {
                let score = Score::Fake(
                    self.estimate_decay * f64::from(cnt)
                        / f64::from(total_counts[prefix_id as usize]),
                );
                Parsed { lex, score }
            })
            .collect();

        subresult.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap().reverse());
        result.list.extend(subresult.into_iter());
    }
}

impl KnownSuffixAnalyzer {
    pub fn possible_prefixes<'m: 'i, 's: 'i, 'i>(
        &self,
        morph: &'m MorphAnalyzer,
        word: &'s str,
    ) -> impl Iterator<Item = (u16, &'m str, &'m PredictionSuffixesDawg)> + 'i {
        use std::ops::Deref;
        morph
            .dict()
            .paradigm_prefixes_rev
            .iter()
            .filter(move |&&(_, ref prefix)| word.starts_with(prefix.deref()))
            .map(move |&(prefix_idx, ref prefix)| {
                (
                    prefix_idx,
                    prefix.deref(),
                    &morph.dict().prediction_suffixes_dawgs[prefix_idx as usize],
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::format::ExplainParsed;
    use crate::util::test::bootstrap_parse_ru;
    use crate::util::test::init_logger;

    #[test]
    fn big_count() {
        init_logger();

        let ks_analyzer = KnownSuffixAnalyzer::default();
        let (morph, mut result, word, word_lower, mut seen_parses) =
            bootstrap_parse_ru("существующой");
        let time = std::time::Instant::now();
        ks_analyzer.parse(morph, &mut result, word, &word_lower, &mut seen_parses);
        log::debug!("Elapsed time: {:0.6} s", time.elapsed().as_secs_f64());

        for (i, parsed) in result.iter().enumerate() {
            log::debug!("{:2}. {}", i + 1, ExplainParsed(morph, parsed));
        }
    }
}
