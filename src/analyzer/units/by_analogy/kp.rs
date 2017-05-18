use analyzer::MorphAnalyzer;
use analyzer::units::abc::Analyzer;
use container::{Parsed, ParseResult, SeenSet};
use container::Lex;
use container::abc::*;
use container::stack::Stack;
use container::stack::StackAffix;
use container::Affix;
use container::AffixKind;
use util::add_parse_if_not_seen;


/// Parse the word by checking if it starts with a known prefix
/// and parsing the reminder.
///
/// Example: псевдокошка -> (псевдо) + кошка.

#[derive(Debug, Clone)]
pub struct KnownPrefixAnalyzer {
    min_reminder_length: isize,
    estimate_decay: f64,
}


impl Default for KnownPrefixAnalyzer {
    fn default() -> Self {
        KnownPrefixAnalyzer {
            min_reminder_length: 3,
            estimate_decay: 0.75,
        }
    }
}


impl Analyzer for KnownPrefixAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, seen_parses: &mut SeenSet) {
        trace!("KnownPrefixAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        for (prefix, unprefixed_word) in self.possible_splits(morph, word_lower) {
            'iter_parses: for parsed in morph.parse(unprefixed_word) {
                let tag = parsed.lex.get_tag(morph);
                if !tag.is_productive() {
                    continue 'iter_parses
                }
                let container = match parsed.lex.stack {
                    Stack::Source(stack) =>
                        StackAffix {
                            stack: stack,
                            affix: Some(Affix {
                                part: prefix.to_string(),
                                kind: AffixKind::KnownPrefix,
                            })
                        },
                    // FIXME (ad #3)
//                    _ => unreachable!(),
                    _ => continue,
                };
                add_parse_if_not_seen(morph, result, seen_parses, Parsed {
                    lex: Lex::from_stack(morph, container),
                    score: parsed.score * self.estimate_decay,
                });
            }
        }
    }
}


impl KnownPrefixAnalyzer {
    fn possible_splits<'m: 'i, 's: 'i, 'i>(&self, morph: &'m MorphAnalyzer, word: &'s str)
        -> impl Iterator<Item = (&'s str, &'s str)> + 'i
    {
        let limit = word.chars().count() as isize - self.min_reminder_length;
        let mut word_prefixes = morph.dict.prediction_prefixes.prefixes(word);
        word_prefixes.sort_by_key(|v| -(v.len() as isize));
        trace!("word_prefixes: {}", word_prefixes.join(", "));
        word_prefixes.into_iter()
            .filter(move |prefix| prefix.chars().count() as isize <= limit)
            .map(move |prefix| {
                let char_len = prefix.chars().count();
                let pos = word.chars().take(char_len).map(char::len_utf8).sum();
                (prefix, &word[pos ..])
//                (prefix, &word[prefix.len() ..])
            })
    }
}
