use std::cmp::Ordering;

use smallvec::SmallVec;

use crate::analyzer::common::parse::Parse;
use crate::analyzer::common::ProbabilityEstimator;
use crate::analyzer::common::Score;
use crate::analyzer::Lang;
use crate::opencorpora::tag::TagReg;
use crate::opencorpora::Dictionary;

#[derive(Debug, Clone)]
pub struct SingleTagProbabilityEstimator {}

impl SingleTagProbabilityEstimator {
    pub fn prob(&self, dict: &Dictionary, word_lower: &str, tag: &TagReg) -> f64 {
        // FIXME avoid allocation
        let dawg_key = format!("{}:{}", word_lower, tag.fmt_int);
        let v = dict
            .p_t_given_w
            .as_ref()
            .and_then(|dawg| dawg.find(&dawg_key))
            .unwrap_or(0);
        f64::from(v) / 1_000_000.0
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

        let probs: SmallVec<[f64; 256]> = parses
            .iter()
            .map(|p| self.prob(morph.dict(), word_lower, morph.get_tag(&p.lex)))
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
            parses.sort_by(|p1, p2| {
                p2.score
                    .value()
                    .partial_cmp(&p1.score.value())
                    .unwrap_or(Ordering::Equal)
            });
        }
    }

    fn apply_to_tags<L: Lang>(
        &self,
        morph: &L,
        _word: &str,
        word_lower: &str,
        tags: &mut [TagReg],
    ) {
        if tags.is_empty() {
            return;
        }

        // FIXME avoid double probability calculation.
        tags.sort_by(|t1: &TagReg, t2: &TagReg| {
            self.prob(morph.dict(), word_lower, t2)
                .partial_cmp(&self.prob(morph.dict(), word_lower, t1))
                .unwrap_or(Ordering::Equal)
        });
    }
}
