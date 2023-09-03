use crate::analyzer::common::parse::Parse;
use crate::analyzer::common::ProbabilityEstimator;
use crate::analyzer::Lang;
use crate::opencorpora::tag::TagReg;

#[derive(Debug, Clone)]
pub struct NoopProbabilityEstimator {}

impl ProbabilityEstimator for NoopProbabilityEstimator {
    fn apply_to_parses<L: Lang>(
        &self,
        _morph: &L,
        _word: &str,
        _word_lower: &str,
        _parses: &mut [Parse],
    ) {
        return;
    }

    fn apply_to_tags<L: Lang>(
        &self,
        _morph: &L,
        _word: &str,
        _word_lower: &str,
        _tags: &mut [TagReg],
    ) {
        return;
    }
}
