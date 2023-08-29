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

pub static RU_LETTERS: [char; 30] = [
    'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С',
    'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Э', 'Ю', 'Я',
];

pub static RU_LETTERS_STR: [&str; 30] = [
    "А", "Б", "В", "Г", "Д", "Е", "Ё", "Ж", "З", "И", "Й", "К", "Л", "М", "Н", "О", "П", "Р", "С",
    "Т", "У", "Ф", "Х", "Ц", "Ч", "Ш", "Щ", "Э", "Ю", "Я",
];

const SCORE: Score = Score::Fake(0.1);

#[derive(Debug, Clone)]
pub struct InitialsAnalyzer {
    pub(crate) tags: Vec<(TagReg, InitialKind)>,
}

impl InitialsAnalyzer {
    pub(in crate::analyzer) fn new(grammemes: &GramRegistry) -> Self {
        use InitialKind::*;

        if cfg!(debug_assertions) {
            assert_eq!(RU_LETTERS.len(), RU_LETTERS_STR.len());

            RU_LETTERS
                .iter()
                .zip(RU_LETTERS_STR.iter())
                .for_each(|(c, s)| {
                    assert_eq!(*c, s.chars().next().unwrap());
                });
        }

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

    pub(in crate::analyzer) fn find_letter(letter: &str) -> Option<&'static str> {
        let mut it = letter.chars();

        let Some(letter) = it.next() else { None? };

        if it.next().is_some() {
            None?
        }

        let index = RU_LETTERS.iter().copied().position(|c| c == letter)?;

        Some(RU_LETTERS_STR[index])
    }
}

impl AnalyzerUnit for InitialsAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("InitialsAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word: "{}", word_lower: "{}" "#, word, word_lower);
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" RU_LETTERS: "{:?}" "#, RU_LETTERS);

        let letter = Self::find_letter(ctx.word());

        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" RU_LETTERS contains word: {:?} "#, letter);

        if let Some(letter) = letter {
            for (tag_idx, &(_, kind)) in self.tags.iter().enumerate() {
                let source = Initial::new(letter, kind, tag_idx as u8);
                let lex = Lex::from_stack(StackSource::from(source));
                ctx.push(Parsed::new(lex, SCORE));
            }
        }
    }
}
