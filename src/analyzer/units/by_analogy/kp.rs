use analyzer::MorphAnalyzer;
use analyzer::units::abc::AnalyzerUnit;
use container::{Affix, AffixKind, Lex, Parsed, ParseResult, SeenSet};
use container::abc::*;
use container::stack::StackAffix;
use util::add_parsed_if_not_seen;


/// Parse the word by checking if it starts with a known prefix
/// and parsing the reminder.
///
/// Example: псевдокошка -> (псевдо) + кошка.

#[derive(Debug, Clone)]
pub struct KnownPrefixAnalyzer {
    min_reminder_length: usize,
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


impl AnalyzerUnit for KnownPrefixAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, seen_parses: &mut SeenSet) {
        trace!("KnownPrefixAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        for (prefix, unprefixed_word) in self.possible_splits(morph, word_lower) {
            'iter_parses: for parsed in morph.parse(unprefixed_word) {
                let tag = parsed.lex.get_tag(morph);
                if !tag.is_productive() {
                    continue 'iter_parses
                }
                let lex_stack = parsed.lex.stack;
                // FIXME is it equivalent for `FIXME (ad #3)` below?
                if lex_stack.particle.is_some() ||
                    lex_stack.stack.right.is_some() ||
                    lex_stack.stack.left.affix.is_some()
                {
                    continue;
                }
                let affix = Some(Affix {
                    part: prefix.to_string(),
                    kind: AffixKind::KnownPrefix,
                });
                let container = StackAffix::new(lex_stack.stack.left.stack, affix);
//                let container = match parsed.lex.stack {
//                    Stack::Source(stack) =>
//                        StackAffix {
//                            stack: stack,
//                            affix: Some(Affix {
//                                part: prefix.to_string(),
//                                kind: AffixKind::KnownPrefix,
//                            })
//                        },
//                    // FIXME (ad #3)
////                    _ => unreachable!(),
//                    _ => continue,
//                };
                let lex = Lex::from_stack(morph, container);
                let score = parsed.score * self.estimate_decay;
                add_parsed_if_not_seen(morph, result, seen_parses, Parsed { lex, score });
            }
        }
    }
}


impl KnownPrefixAnalyzer {
    fn possible_splits<'m: 'i, 's: 'i, 'i>(&self, morph: &'m MorphAnalyzer, word: &'s str)
        -> impl Iterator<Item = (&'s str, &'s str)> + 'i
    {
        let word_len = word.chars().count();
        assert!(word_len >= self.min_reminder_length);
        let limit = word_len - self.min_reminder_length;
        let word_prefixes = morph.dict.prediction_prefixes.sorted_prefixes(word);
        trace!("word_prefixes: {}", word_prefixes.join(", "));
        word_prefixes.into_iter()
            .map(move |prefix| (prefix.chars().count(), prefix))
            .filter(move |&(char_len, _prefix)| char_len <= limit)
            .map(move |(char_len, prefix)| {
                let pos = word.chars().take(char_len).map(char::len_utf8).sum();
                (prefix, &word[pos ..])
                // FIXME why not this?
//                (prefix, &word[prefix.len() ..])
            })
    }
}
