use std::collections::BTreeSet;
use std::iter::FromIterator;

use analyzer::MorphAnalyzer;
use analyzer::units::abc::Analyzer;
use container::{Initials, InitialsKind};
use container::{Parsed, ParseResult, SeenSet};
use container::{Lex, Score};
use container::stack::StackSource;
use opencorpora::OpencorporaTagReg;


lazy_static! {
    pub static ref LETTERS: BTreeSet<&'static str> = {
        let set = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЭЮЯ".split("").filter(|v| !v.is_empty());
        BTreeSet::from_iter(set)
    };
}

#[derive(Debug, Clone)]
pub struct InitialsAnalyzer {
    pub tags: Vec<OpencorporaTagReg>,
}


impl Default for InitialsAnalyzer {
    fn default() -> Self {
        InitialsAnalyzer {
            tags: {
                let mut result: Vec<_> = Vec::new();
                for kind in &["Name", "Patr"] {
                    for gender in &["masc", "femn"] {
                        for case in &["nomn", "gent", "datv", "accs", "ablt", "loct"] {
                            result.push(
                                OpencorporaTagReg::new(
                                    format!("NOUN,anim,{gender},Sgtm,{kind},Fixd,Abbr,Init sing,{case}",
                                            kind = kind, gender = gender, case = case)
                                        .as_str()
                                ))
                        }
                    }
                }
                result
            }
        }
    }
}


impl Analyzer for InitialsAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("AbbreviatedFirstNameAnalyzer::parse()");
        trace!(r#" word: "{}", word_lower: "{}" "#, word, word_lower);
        trace!(r#" LETTERS: "{:?}" "#, LETTERS.iter().cloned().collect::<Vec<&str>>().join(", "));
        trace!(r#" LETTERS contains word: "{}" "#, LETTERS.contains(word));

        if LETTERS.contains(word) {
            for tag_idx in 0..self.tags.len() {
                let container = Initials {
                    letter: word.to_owned(),
                    kind: if tag_idx < self.tags.len() / 2
                        { InitialsKind::FirstName } else { InitialsKind::Patronym },
                    tag_idx: tag_idx as u8,
                };
                result.push(Parsed {
                    lex: Lex::from_stack(morph, StackSource::from(container)),
                    score: Score::Fake(0.1),
                });
            }
        }
    }
}
