use std::collections::BTreeSet;
use std::iter::FromIterator;

use crate::analyzer::context::ParseContext;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Initial;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Score;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::kind::InitialKind;
use crate::opencorpora::tag::TagReg;

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
    pub tags: Vec<(TagReg, InitialKind)>,
}

impl InitialsAnalyzer {
    pub(in crate::analyzer) fn new(grammemes: &GramRegistry) -> Self {
        use InitialKind::*;

        let mut tags: Vec<_> = Vec::new();
        for kind in [FirstName, Patronym] {
            for gender in ["masc", "femn"] {
                for case in ["nomn", "gent", "datv", "accs", "ablt", "loct"] {
                    let tag = TagReg::from_fmt_int(
                        grammemes,
                        format!(
                            "NOUN,anim,{gender},Sgtm,{kind},Fixd,Abbr,Init sing,{case}",
                            kind = kind.gram_int(),
                            gender = gender,
                            case = case
                        ),
                    );
                    tags.push((tag, kind));
                }
            }
        }
        InitialsAnalyzer { tags }
    }
}

impl AnalyzerUnit for InitialsAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("AbbreviatedFirstNameAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word: "{}", word_lower: "{}" "#, word, word_lower);
        #[cfg(feature = "trace_analyzer")]
        log::trace!(
            r#" LETTERS: "{:?}" "#,
            LETTERS.iter().cloned().collect::<Vec<&str>>().join(", ")
        );
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" LETTERS contains word: "{}" "#, LETTERS.contains(word));

        if let Some(&letter) = LETTERS.get(ctx.word()) {
            for (tag_idx, &(_, kind)) in self.tags.iter().enumerate() {
                let source = Initial {
                    letter,
                    kind,
                    tag_idx: tag_idx as u8,
                };
                let lex = Lex::from_stack(StackSource::from(source));
                ctx.push(Parsed::new(lex, SCORE));
            }
        }
    }
}
