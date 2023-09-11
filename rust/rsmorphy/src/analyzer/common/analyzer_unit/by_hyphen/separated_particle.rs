use std::marker::PhantomData;

use crate::analyzer::common::HyphenSeparatedParticle;
use crate::analyzer::common::ParseContext;
use crate::analyzer::Lang;
use crate::analyzer::Parse;

const SCORE_MULTIPLIER: f64 = 0.9;

/// Parse the word by analyzing it without
/// a particle after a hyphen.
///
/// Example: смотри-ка -> смотри + "-ка".
///
/// .. note::
///
/// This analyzer doesn't remove particles from the result
/// so for normalization you may need to handle
/// particles at tokenization level.

#[derive(Debug)]
pub struct HyphenSeparatedParticleAnalyzer<L: Lang> {
    particles_after_hyphen: &'static [&'static str],
    score_multiplier: f64,
    _lang: PhantomData<L>,
}

impl<L: Lang> HyphenSeparatedParticleAnalyzer<L> {
    pub fn new(
        particles_after_hyphen: &'static [&'static str],
        score_multiplier: Option<f64>,
    ) -> Self {
        HyphenSeparatedParticleAnalyzer {
            particles_after_hyphen,
            score_multiplier: score_multiplier.unwrap_or(SCORE_MULTIPLIER),
            _lang: PhantomData,
        }
    }

    pub fn parse(&self, ctx: &mut ParseContext<L>) {
        #[cfg(feature = "log_trace_analyzer")]
        log::trace!("HyphenSeparatedParticleAnalyzer::parse(); {ctx:?}");

        let morph = ctx.morph();
        let word_lower = ctx.word_lower().clone();
        let mut buffer = morph.make_buffer();

        'particle: for &particle in self.particles_after_hyphen {
            if word_lower.len() <= particle.len() || !word_lower.ends_with(particle) {
                continue 'particle;
            };

            #[cfg(feature = "log_trace_analyzer")]
            log::trace!(r#" particle: "{}" "#, particle);

            let unsuffixed_word = &word_lower[..word_lower.len() - particle.len()];

            #[cfg(feature = "log_trace_analyzer")]
            log::trace!(r#" unsuffixed_word: "{}" "#, unsuffixed_word);

            'subparse: for Parse { mut lex, score } in buffer.parse(unsuffixed_word).drain() {
                #[cfg(feature = "log_trace_analyzer")]
                log::trace!(r#" subparse: {score:?}, {lex:?} "#);

                // If a word ends with with one of the particles, it can't ends with an another.
                if lex.particle.is_some() {
                    continue 'subparse;
                };

                lex.particle = Some(HyphenSeparatedParticle::new(particle));
                let score = score * self.score_multiplier;

                ctx.push(Parse::new(lex, score));
            }
        }
    }
}
