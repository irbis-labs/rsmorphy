use std::cmp::Ordering;

use crate::estimator::ProbabilityEstimator;
use crate::opencorpora::tag::TagReg;
use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SingleTagProbabilityEstimator {}

impl SingleTagProbabilityEstimator {
    pub fn prob(self, morph: &MorphAnalyzer, word_lower: &str, tag: &TagReg) -> f64 {
        // FIXME avoid allocation
        let dawg_key = format!("{}:{}", word_lower, tag.fmt_int);
        f64::from(morph.dict().p_t_given_w.find(&dawg_key).unwrap_or(0)) / 1_000_000.0
    }
}

impl ProbabilityEstimator for SingleTagProbabilityEstimator {
    fn apply_to_parses(
        &self,
        morph: &MorphAnalyzer,
        _word: &str,
        word_lower: &str,
        parses: &mut [Parsed],
    ) {
        if parses.is_empty() {
            return;
        }

        let probs: Vec<f64> = parses
            .iter()
            .map(|p: &Parsed| self.prob(morph, word_lower, p.lex.get_tag(morph)))
            .collect();

        let prob_sum: f64 = probs.iter().copied().sum();
        if prob_sum.abs() < f64::EPSILON {
            // no P(t|w) information is available; return normalized estimate
            let parses_sum: f64 = parses.iter().map(|p: &Parsed| p.score.value()).sum();
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
            parses.sort_by(|p1: &Parsed, p2: &Parsed| {
                p2.score
                    .value()
                    .partial_cmp(&p1.score.value())
                    .unwrap_or(Ordering::Equal)
            });
        }
    }

    fn apply_to_tags(
        &self,
        morph: &MorphAnalyzer,
        _word: &str,
        word_lower: &str,
        tags: &mut [TagReg],
    ) {
        if tags.is_empty() {
            return;
        }

        // FIXME avoid double probability calculation.
        tags.sort_by(|t1: &TagReg, t2: &TagReg| {
            self.prob(morph, word_lower, t2)
                .partial_cmp(&self.prob(morph, word_lower, t1))
                .unwrap_or(Ordering::Equal)
        });
    }
}
