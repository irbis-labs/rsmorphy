use crate::opencorpora::tag::TagReg;
use crate::MorphAnalyzer;
use crate::Parsed;

mod single_tag;

pub use self::single_tag::SingleTagProbabilityEstimator;

pub trait ProbabilityEstimator {
    fn apply_to_parses(
        &self,
        morph: &MorphAnalyzer,
        word: &str,
        word_lower: &str,
        parses: &mut [Parsed],
    );

    fn apply_to_tags(
        &self,
        morph: &MorphAnalyzer,
        word: &str,
        word_lower: &str,
        tags: &mut [TagReg],
    );
}

impl<'a, T> ProbabilityEstimator for &'a T
where
    T: ProbabilityEstimator,
{
    fn apply_to_parses(
        &self,
        morph: &MorphAnalyzer,
        word: &str,
        word_lower: &str,
        parses: &mut [Parsed],
    ) {
        (*self).apply_to_parses(morph, word, word_lower, parses)
    }

    fn apply_to_tags(
        &self,
        morph: &MorphAnalyzer,
        word: &str,
        word_lower: &str,
        tags: &mut [TagReg],
    ) {
        (*self).apply_to_tags(morph, word, word_lower, tags)
    }
}
