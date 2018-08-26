use std::cmp::Ordering;

use opencorpora::OpencorporaTagReg;
use prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct SingleTagProbabilityEstimator {}

impl SingleTagProbabilityEstimator {
    pub fn prob(self, morph: &MorphAnalyzer, word_lower: &str, tag: &OpencorporaTagReg) -> f64 {
        let dawg_key = format!("{}:{}", word_lower, tag.string);
        f64::from(morph.dict.p_t_given_w.find(&dawg_key).unwrap_or(0)) / 1_000_000.0
    }

    pub fn apply_to_parses(
        self,
        morph: &MorphAnalyzer,
        _word: &str,
        word_lower: &str,
        parses: &mut Vec<Parsed>,
    ) {
        if parses.is_empty() {
            return;
        }

        let probs: Vec<f64> = parses
            .iter()
            .map(|p: &Parsed| self.prob(morph, word_lower, p.lex.get_tag(morph)))
            .collect();

        if probs.iter().cloned().sum::<f64>() == 0.0 {
            // no P(t|w) information is available; return normalized estimate
            let k = 1.0 / parses.iter().map(|p: &Parsed| p.score.value()).sum::<f64>();
            for p in parses {
                p.score = p.score * k;
            }
        } else {
            for (ref mut p, prob) in parses.iter_mut().zip(probs.into_iter()) {
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

    pub fn apply_to_tags(
        self,
        morph: &MorphAnalyzer,
        _word: &str,
        word_lower: &str,
        tags: &mut Vec<OpencorporaTagReg>,
    ) {
        if tags.is_empty() {
            return;
        }

        tags.sort_by(|t1: &OpencorporaTagReg, t2: &OpencorporaTagReg| {
            self.prob(morph, word_lower, t2)
                .partial_cmp(&self.prob(morph, word_lower, t1))
                .unwrap_or(Ordering::Equal)
        });
    }
}
