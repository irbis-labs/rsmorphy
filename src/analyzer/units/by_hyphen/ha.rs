use crate::analyzer::context::ParseContext;
use crate::analyzer::unit::AnalyzerUnit;
use crate::container::abc::*;
use crate::container::stack::StackSource;
use crate::container::HyphenAdverb;
use crate::container::Lex;
use crate::container::Parsed;
use crate::container::Score;
use crate::container::WordStruct;
use crate::opencorpora::kind::Case;
use crate::opencorpora::kind::Number;
use crate::opencorpora::kind::PartOfSpeach;
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
        #[cfg(trace_analyzer)]
        log::trace!("HyphenAdverbAnalyzer::parse()");
        #[cfg(trace_analyzer)]
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
                match (tag.pos, tag.number, tag.case) {
                    (Some(PartOfSpeach::Adjf), Some(Number::Sing), Some(Case::Datv)) => true,
                    _ => false,
                }
            })
            .for_each(|parsed: Parsed| {
                let word = WordStruct::new(ctx.word_lower(), parsed.lex.is_known());
                let lex = Lex::from_stack(morph, StackSource::new(HyphenAdverb::new(word)));
                ctx.push(Parsed::new(lex, HA_SCORE));
            });
    }
}
