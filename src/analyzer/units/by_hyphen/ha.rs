use crate::{
    analyzer::{units::abc::*, MorphAnalyzer},
    container::{
        abc::*, stack::StackSource, HyphenAdverb, Lex, ParseResult, Parsed, Score, SeenSet,
        WordStruct,
    },
    opencorpora::{
        kind::{Case, Number, PartOfSpeach},
        OpencorporaTagReg,
    },
};

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
            tag: OpencorporaTagReg::new("ADVB"),
        }
    }
}

impl AnalyzerUnit for HyphenAdverbAnalyzer {
    fn parse(
        &self,
        morph: &MorphAnalyzer,
        result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        _seen_parses: &mut SeenSet,
    ) {
        log::trace!("HyphenAdverbAnalyzer::parse()");
        log::trace!(r#" word: "{}", word_lower: "{}" "#, word, word_lower);

        if word.chars().count() < 5 || !word.starts_with(HA_PREFIX) {
            return;
        }

        morph
            .parse(&word[HA_PREFIX.len()..])
            .into_iter()
            .filter(|parsed| {
                let tag = parsed.lex.get_tag(morph);
                match (tag.pos, tag.number, tag.case) {
                    (Some(PartOfSpeach::Adjf), Some(Number::Sing), Some(Case::Datv)) => true,
                    _ => false,
                }
            })
            .for_each(|parsed: Parsed| {
                let word = WordStruct::new(word_lower, parsed.lex.is_known());
                let lex = Lex::from_stack(morph, StackSource::new(HyphenAdverb::new(word)));
                result.push(Parsed::new(lex, HA_SCORE));
            });
    }
}
