use std::{borrow::Cow, collections::BTreeSet, iter::FromIterator};

use crate::{
    analyzer::{units::abc::AnalyzerUnit, MorphAnalyzer},
    container::{
        stack::StackSource, Initials, InitialsKind, Lex, ParseResult, Parsed, Score, SeenSet,
    },
    opencorpora::OpencorporaTagReg,
};

lazy_static::lazy_static! {
    #[derive(Debug)]
    pub static ref LETTERS: BTreeSet<&'static str> = {
        let set = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЭЮЯ".split("").filter(|v| !v.is_empty());
        BTreeSet::from_iter(set)
    };
}

const SCORE: Score = Score::Fake(0.1);

#[derive(Debug, Clone)]
pub struct InitialsAnalyzer {
    pub tags: Vec<(OpencorporaTagReg, InitialsKind)>,
}

impl Default for InitialsAnalyzer {
    fn default() -> Self {
        InitialsAnalyzer {
            tags: {
                let mut result: Vec<_> = Vec::new();
                for &kind in &["Name", "Patr"] {
                    for &gender in &["masc", "femn"] {
                        for &case in &["nomn", "gent", "datv", "accs", "ablt", "loct"] {
                            let tag = OpencorporaTagReg::new(format!(
                                "NOUN,anim,{gender},Sgtm,{kind},Fixd,Abbr,Init sing,{case}",
                                kind = kind,
                                gender = gender,
                                case = case
                            ));
                            let kind = match kind {
                                "Name" => InitialsKind::FirstName,
                                "Patr" => InitialsKind::Patronym,
                                _ => unreachable!(),
                            };
                            result.push((tag, kind));
                        }
                    }
                }
                result
            },
        }
    }
}

impl AnalyzerUnit for InitialsAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        _seen_parses: &mut SeenSet,
    ) {
        log::trace!("AbbreviatedFirstNameAnalyzer::parse()");
        log::trace!(r#" word: "{}", word_lower: "{}" "#, word, word_lower);
        log::trace!(
            r#" LETTERS: "{:?}" "#,
            LETTERS.iter().cloned().collect::<Vec<&str>>().join(", ")
        );
        log::trace!(r#" LETTERS contains word: "{}" "#, LETTERS.contains(word));

        if let Some(&letter) = LETTERS.get(word) {
            for (tag_idx, &(_, kind)) in self.tags.iter().enumerate() {
                let tag_idx = tag_idx as u8;
                let letter = Cow::from(letter);
                let container = Initials {
                    letter,
                    kind,
                    tag_idx,
                };
                let lex = Lex::from_stack(morph, StackSource::from(container));
                result.push(Parsed::new(lex, SCORE));
            }
        }
    }
}
