use crate::analyzer::context::ParseContext;
use crate::analyzer::model::abc::*;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::HyphenAdverb;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Score;
use crate::analyzer::model::WordStruct;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::kind::Case;
use crate::opencorpora::kind::Number;
use crate::opencorpora::kind::PartOfSpeech;
use crate::opencorpora::tag::TagReg;

const RU_HA_PREFIX: &str = "по-";
const RU_HA_PREFIX_LEN: usize = 3;
const RU_MIN_WORD_LEN: usize = 5;
const RU_MIN_REMNANT_LEN: usize = RU_MIN_WORD_LEN - RU_HA_PREFIX_LEN;
const SCORE_MULTIPLIER: f64 = 0.7;

/// Detect adverbs that starts with "по-".
///
/// Example: по-западному

#[derive(Debug, Clone)]
pub struct HyphenAdverbAnalyzer {
    pub tag: TagReg,
    score_multiplier: f64,
}

impl HyphenAdverbAnalyzer {
    pub fn new(grammemes: &GramRegistry) -> Self {
        HyphenAdverbAnalyzer {
            tag: TagReg::from_fmt_int(grammemes, "ADVB"),
            score_multiplier: SCORE_MULTIPLIER,
        }
    }
}

impl AnalyzerUnit for HyphenAdverbAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("HyphenAdverbAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word: "{}", word_lower: "{}" "#, word, word_lower);

        let morph = ctx.morph();
        let word = ctx.word();
        if !word.starts_with(RU_HA_PREFIX) {
            return;
        }
        let remnant = &word[RU_HA_PREFIX.len()..];
        if remnant.chars().count() < RU_MIN_REMNANT_LEN {
            return;
        }

        // TODO simplify (e.g., avoid estimation)
        morph
            .parse(remnant)
            .into_iter()
            .filter(|parsed| {
                let tags = &parsed.lex.get_tag(morph).tags;
                matches!(
                    (tags.pos, tags.number, tags.case),
                    (
                        Some(PartOfSpeech::AdjectiveFull),
                        Some(Number::Sing),
                        Some(Case::Datv)
                    )
                )
            })
            .for_each(|parsed: Parsed| {
                let word = WordStruct::new(ctx.word_lower(), parsed.lex.is_known());
                let lex = Lex::from_stack(StackSource::new(HyphenAdverb::new(word)));
                ctx.push(Parsed::new(lex, Score::Fake(self.score_multiplier)));
            });
    }
}
