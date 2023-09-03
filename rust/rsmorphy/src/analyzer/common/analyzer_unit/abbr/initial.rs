use crate::analyzer::common::util::Letters;
use crate::analyzer::common::Initial;
use crate::analyzer::common::ParseContext;
use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::kind::InitialKind;
use crate::opencorpora::tag::TagReg;

const SCORE: Score = Score::Fake(0.1);

#[derive(Debug)]
pub struct InitialsAnalyzer<L: Lang> {
    pub tags: Vec<(TagReg, InitialKind)>,
    pub letters: Letters,
    pub score: Score,
    _lang: std::marker::PhantomData<L>,
}

impl<L: Lang> InitialsAnalyzer<L> {
    pub fn new(grammemes: &GramRegistry, letters: Letters, score: Option<Score>) -> Self {
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

        InitialsAnalyzer {
            tags,
            letters,
            score: score.unwrap_or(SCORE),
            _lang: std::marker::PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("InitialsAnalyzer::parse(); {ctx:?}");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" LETTERS: "{:?}" "#, self.letters);

        let letter = self.letters.find(ctx.word());

        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" LETTERS contains word: {:?} "#, letter);

        if let Some(letter) = letter {
            for (tag_idx, &(_, kind)) in self.tags.iter().enumerate() {
                let lex = Lex::from_base(Initial::new(letter, kind, tag_idx as u8));
                ctx.push(Parse::new(lex, SCORE));
            }
        }
    }
}
