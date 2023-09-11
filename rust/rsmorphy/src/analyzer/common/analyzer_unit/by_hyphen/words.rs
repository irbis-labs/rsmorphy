use std::marker::PhantomData;

use crate::analyzer::common::ParseContext;
use crate::analyzer::Lang;
use crate::data_struct::prefixes::Prefixes;

const SCORE_MULTIPLIER: f64 = 0.75;

/// Parse the word by parsing its hyphen-separated parts.
///
/// Examples:
///
/// * интернет-магазин -> "интернет-" + магазин
/// * человек-гора -> человек + гора

#[derive(Debug)]
pub struct HyphenatedWordsAnalyzer<L: Lang> {
    #[allow(dead_code)]
    skip_prefixes: Prefixes,
    #[allow(dead_code)]
    score_multiplier: f64,
    _lang: PhantomData<L>,
}

impl<L: Lang> HyphenatedWordsAnalyzer<L> {
    pub fn new(skip_prefixes: Prefixes, score_multiplier: Option<f64>) -> Self {
        HyphenatedWordsAnalyzer {
            skip_prefixes,
            score_multiplier: score_multiplier.unwrap_or(SCORE_MULTIPLIER),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, _ctx: &mut ParseContext<L>) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("HyphenatedWordsAnalyzer::parse(); {_ctx:?}");

        // FIXME unimplemented
        return;

        //        let splitted: Vec<&str> = word_lower.split("-").collect();
        //        if splitted.len() > 2 {
        //            return
        //        }
        //        let (left, right) = (morph.parse(splitted[0]), morph.parse(splitted[1]));

        //        for particle in PARTICLES_AFTER_HYPHEN.iter() {
        //            if word_lower.len() <= particle.len() || !word_lower.ends_with(particle) {
        //                continue
        //            };
        //            trace!(r#" particle: "{}" "#, particle);
        //            let unsuffixed_word = &word_lower[.. word_lower.len() - particle.len()];
        //            trace!(r#" unsuffixed_word: "{}" "#, unsuffixed_word);
        //
        //            'subparse: for parsed in morph.parse(unsuffixed_word) {
        //                trace!(r#" parsed: {:?} "#, parsed);
        //                let stack: StackHyphenated = match parsed.lex.stack {
        //                    // If a word ends with with one of the particles, it can't ends with an another.
        //                    Stack::HSP(_)               => continue 'subparse,
        //                    Stack::Hyphenated(ref v)    => v.clone(),
        //                    Stack::Affix(ref v)         => v.clone().into(),
        //                    Stack::Source(ref v)        => v.clone().into(),
        //                };
        //                let container = StackParticle::Particle(stack, HyphenSeparatedParticle {
        //                    particle: particle.to_string(),
        //                });
        //                result.push(Parsed {
        //                    lex: Lex::from_stack(morph, container),
        //                    score: Score::Real(1.0),
        //                });
        //            }
        //        }
    }
}
