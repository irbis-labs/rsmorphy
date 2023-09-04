use std::cmp::Ordering;

use smallvec::SmallVec;

use crate::analyzer::common::parse::Parse;
use crate::analyzer::common::ProbabilityEstimator;
use crate::analyzer::common::Score;
use crate::analyzer::Lang;
use crate::dawg::WordTagDawgKey;
use crate::model::tag::TagReg;
use crate::model::Dictionary;

#[derive(Debug, Clone)]
pub struct SingleTagProbabilityEstimator {}

impl SingleTagProbabilityEstimator {
    #[inline(always)]
    pub fn prob(&self, dict: &Dictionary, word_lower: &str, tag: &TagReg) -> f64 {
        self.prob_(dict, word_lower, tag).unwrap_or(0.0)
    }

    fn prob_(&self, dict: &Dictionary, word_lower: &str, tag: &TagReg) -> Option<f64> {
        let dawg = dict.p_t_given_w.as_ref()?;
        let dawg_key = WordTagDawgKey::new(word_lower, &tag.fmt_int)?;
        let prob: f64 = dawg.find(&dawg_key)?.as_u32().into();
        Some(prob / 1_000_000.0)
    }
}

impl ProbabilityEstimator for SingleTagProbabilityEstimator {
    fn apply_to_parses<L: Lang>(
        &self,
        morph: &L,
        _word: &str,
        word_lower: &str,
        parses: &mut [Parse],
    ) {
        if parses.is_empty() {
            return;
        }

        let dict = morph.dict();
        let probs: SmallVec<[f64; 256]> = parses
            .iter()
            .map(|p| self.prob(dict, word_lower, morph.get_tag(&p.lex)))
            .collect();

        let prob_sum: f64 = probs.iter().copied().sum();
        if prob_sum.abs() < f64::EPSILON {
            // no P(t|w) information is available; return normalized estimate
            let parses_sum: f64 = parses.iter().map(|p| p.score.value()).sum();
            if parses_sum.abs() < f64::EPSILON {
                // never should happen, but just in case
                let score = Score::Fake(1.0 / parses.len() as f64);
                for p in parses {
                    p.score = score;
                }
            } else {
                let k = 1.0 / parses_sum;
                for p in parses {
                    p.score = p.score * k;
                }
            }
        } else {
            for (p, prob) in parses.iter_mut().zip(probs.into_iter()) {
                p.score = Score::Real(prob);
            }
            parses.sort_by_key(|parse| F(-parse.score.value()));
        }
    }

    fn apply_to_tags<L: Lang>(
        &self,
        morph: &L,
        _word: &str,
        word_lower: &str,
        tags: &mut [TagReg],
    ) {
        let dict = morph.dict();
        tags.sort_by_cached_key(|tag: &TagReg| F(-self.prob(dict, word_lower, tag)));
    }
}

#[derive(Debug, Clone, Copy, PartialOrd)]
struct F(f64);

impl PartialEq for F {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for F {}

impl Ord for F {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}
