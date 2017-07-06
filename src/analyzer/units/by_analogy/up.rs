use analyzer::MorphAnalyzer;
use analyzer::units::abc::Analyzer;
use container::{Parsed, ParseResult, SeenSet};
use container::Lex;
use container::abc::*;
use container::stack::StackAffix;
use container::Affix;
use container::AffixKind;
use util::add_parse_if_not_seen;
use util::word_splits;


// FIXME unused const MIN_REMINDER_LENGTH: isize = 3;


/// Parse the word by parsing only the word suffix
/// (with restrictions on prefix & suffix lengths).
///
/// Example: байткод -> (байт) + код

#[derive(Debug, Clone)]
pub struct UnknownPrefixAnalyzer {
    estimate_decay: f64,
}


impl Default for UnknownPrefixAnalyzer {
    fn default() -> Self {
        UnknownPrefixAnalyzer {
            estimate_decay: 0.5,
        }
    }
}


impl Analyzer for UnknownPrefixAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, seen_parses: &mut SeenSet) {
        trace!("UnknownPrefixAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        for (prefix, unprefixed_word) in word_splits(word_lower, None, None) {
            let mut subresult = ParseResult::new();
            morph.units.dictionary.parse(morph, &mut subresult, unprefixed_word, unprefixed_word, seen_parses);
            'iter_parses: for parsed in subresult {
                let tag = parsed.lex.get_tag(morph);
                if !tag.is_productive() {
                    continue 'iter_parses
                }
                let container = StackAffix {
                    stack: parsed.lex.stack.stack.left.stack,
                    affix: Some(Affix {
                        part: prefix.to_string(),
                        kind: AffixKind::UnknownPrefix,
                    })
                };
                add_parse_if_not_seen(morph, result, seen_parses, Parsed {
                    lex: Lex::from_stack(morph, container),
                    score: parsed.score * self.estimate_decay,
                });
            }
        }
    }
}




