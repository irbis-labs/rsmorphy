use crate::analyzer::common::parse::Parse;
use crate::analyzer::Lang;
use crate::model::tag::TagReg;

mod single_tag;

pub use self::single_tag::SingleTagProbabilityEstimator;

pub trait ProbabilityEstimator {
    fn apply_to_parses<L: Lang>(
        &self,
        morph: &L,
        word: &str,
        word_lower: &str,
        parses: &mut [Parse],
    );

    fn apply_to_tags<L: Lang>(&self, morph: &L, word: &str, word_lower: &str, tags: &mut [TagReg]);
}

impl<'a, T> ProbabilityEstimator for &'a T
where
    T: ProbabilityEstimator,
{
    fn apply_to_parses<L: Lang>(
        &self,
        morph: &L,
        word: &str,
        word_lower: &str,
        parses: &mut [Parse],
    ) {
        (*self).apply_to_parses(morph, word, word_lower, parses)
    }

    fn apply_to_tags<L: Lang>(&self, morph: &L, word: &str, word_lower: &str, tags: &mut [TagReg]) {
        (*self).apply_to_tags(morph, word, word_lower, tags)
    }
}
