use crate::analyzer::context::ParseContext;
use crate::analyzer::model::abc::*;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::HyphenAdverb;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::model::Score;
use crate::analyzer::model::WordStruct;
use crate::analyzer::unit::AnalyzerUnit;
use crate::opencorpora::kind::Case;
use crate::opencorpora::kind::Number;
use crate::opencorpora::kind::PartOfSpeech;
use crate::opencorpora::OpencorporaTagReg;

const HA_PREFIX: &str = "по-";
const HA_SCORE: Score = Score::Fake(0.7);

/// Detect adverbs that starts with "по-".
///
/// Example: по-западному

#[derive(Debug, Clone)]
pub struct HyphenAdverbAnalyzer {
    pub tag: OpencorporaTagReg,
}

impl Default for HyphenAdverbAnalyzer {
    fn default() -> Self {
        HyphenAdverbAnalyzer {
            tag: OpencorporaTagReg::from_fmt_int("ADVB"),
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
        if !word.starts_with(HA_PREFIX) || word.chars().count() < 5 {
            return;
        }
        let remnant = &word[HA_PREFIX.len()..];
        if remnant.chars().count() < 2 {
            return;
        }

        morph
            .parse(remnant)
            .into_iter()
            .filter(|parsed| {
                let tag = parsed.lex.get_tag(morph);
                matches!(
                    (tag.pos, tag.number, tag.case),
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
                ctx.push(Parsed::new(lex, HA_SCORE));
            });
    }
}
