use std::borrow::Cow;

use analyzer::MorphAnalyzer;
use analyzer::units::abc::Analyzer;
use container::{Dictionary, Word};
use container::{Parsed, ParseResult, SeenSet, Seen};
use container::{Lex, Score};
use container::stack::StackAffix;
use container::Affix;
use container::AffixKind;
use opencorpora::dictionary::PredictionSuffixesDawg;
use opencorpora::dictionary::HHH;


/// Parse the word by checking how the words with similar suffixes
/// are parsed.
///
/// Example: бутявкать -> ...вкать

#[derive(Debug, Clone)]
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


impl Analyzer for KnownSuffixAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, seen_parses: &mut SeenSet) {
        trace!("KnownSuffixAnalyzer::parse()");
        trace!(r#" word: "{}", word_lower: "{}" "#, word, word_lower);

        let char_len: usize = word_lower.chars().count();

        if char_len < self.min_word_length {
            return
        }

        let mut subresult: Vec<(u16, u16, Lex)> = Vec::new();

        // TODO BTreeMap
        let mut total_counts: Vec<u16> = Vec::with_capacity(morph.dict.paradigm_prefixes.len());
        total_counts.resize(morph.dict.paradigm_prefixes.len(), 1);

        for (prefix_id, prefix, suffixes_dawg) in self.possible_prefixes(morph, word_lower) {
            trace!(r#" prefix_id: {}, prefix: "{}" "#, prefix_id, prefix);

            'iter_splits: for &i in &morph.dict.prediction_splits {
                if i > char_len - 1 {
                    continue
                }

                let pos = word_lower.chars().take(char_len - i).map(char::len_utf8).sum();
                trace!("i: {}, pos: {}", i, pos);

                let (word_start, word_end) = (&word_lower[.. pos], &word_lower[pos ..]);
                trace!("word_start: {}, word_end: {}", word_start, word_end);

                let para_data = suffixes_dawg.similar_items(word_end, &morph.dict.char_substitutes);
                for (fixed_suffix, parses) in para_data {
                    trace!("fixed_suffix: {}", fixed_suffix);

                    let fixed_word: Cow<str> = if fixed_suffix == word_end {
                        Cow::from(word_lower)
                    } else {
                        Cow::from(format!("{}{}", word_start, fixed_suffix))
                    };

                    'iter_parses: for HHH(cnt, para_id, idx) in parses {
                        let tag = morph.dict.build_tag_info(para_id, idx);

                        if !tag.is_productive() {
                            continue 'iter_parses
                        }

                        total_counts[prefix_id as usize] += cnt;

                        let seen = Seen {
                            word: fixed_word.clone(),
                            tag: tag,
                            para_id: Some(para_id)
                        };

                        if !seen_parses.insert(&seen) {
                            continue 'iter_parses;
                        }

                        let container = StackAffix {
                            stack: Dictionary {
                                word_lower: Word {
                                    is_known: false,
                                    word: seen.word.into_owned(),
                                },
                                para_id: para_id,
                                idx: idx,
                            }.into(),
                            affix: Some(Affix {
                                part: fixed_suffix.to_string(),
                                kind: AffixKind::KnownSuffix,
                            })
                        };

                        subresult.push((cnt, prefix_id, Lex::from_stack(morph, container)));
                    }
                    if total_counts[prefix_id as usize] > 1 {
                        break 'iter_splits;
                    }
                }
            }
        }

        let mut subresult: ParseResult = subresult.into_iter()
            .map(|(cnt, prefix_id, lex)| Parsed {
                lex: lex,
                score: Score::Fake(
                    self.estimate_decay * cnt as f64 / total_counts[prefix_id as usize] as f64)
            })
            .collect();

        // sort_by(|a, b| a.partial_cmp(b).unwrap())
        // |b, a| ==> .rev()
        subresult.sort_by(|z, a| a.score.value().partial_cmp(&z.score.value()).unwrap());
        for parsed in subresult {
            result.push(parsed);
        }
    }
}

impl KnownSuffixAnalyzer {
    pub fn possible_prefixes<'m: 'i, 's: 'i, 'i>(&self, morph: &'m MorphAnalyzer, word: &'s str)
        -> impl Iterator<Item = (u16, &'m str, &'m PredictionSuffixesDawg)> + 'i
    {
        morph.dict.paradigm_prefixes_rev.iter()
            .filter(move |&&(_prefix_idx, ref prefix)| word.starts_with(prefix.as_str()))
            .map(move |&(prefix_idx, ref prefix)| (
                prefix_idx,
                prefix.as_str(),
                &morph.dict.prediction_suffixes_dawgs[prefix_idx as usize]
            ) )
    }
}
