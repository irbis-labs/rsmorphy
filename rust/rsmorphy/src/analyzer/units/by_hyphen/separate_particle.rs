use crate::analyzer::context::ParseContext;
use crate::analyzer::model::stack::StackParticle;
use crate::analyzer::model::HyphenSeparatedParticle;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Parsed;
use crate::analyzer::unit::AnalyzerUnit;

// TODO move into `Dictionary`
pub static RU_PARTICLES_AFTER_HYPHEN: &[&str] =
    &["-то", "-ка", "-таки", "-де", "-тко", "-тка", "-с", "-ста"];

pub const SCORE_MULTIPLIER: f64 = 0.9;

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

#[derive(Debug, Clone, Copy)]
pub struct HyphenSeparatedParticleAnalyzer {
    particles_after_hyphen: &'static [&'static str],
    score_multiplier: f64,
}

impl HyphenSeparatedParticleAnalyzer {
    pub(in crate::analyzer) fn new() -> Self {
        HyphenSeparatedParticleAnalyzer {
            particles_after_hyphen: RU_PARTICLES_AFTER_HYPHEN,
            score_multiplier: SCORE_MULTIPLIER,
        }
    }
}

impl AnalyzerUnit for HyphenSeparatedParticleAnalyzer {
    fn parse(&self, ctx: &mut ParseContext) {
        #[cfg(feature = "trace_analyzer")]
        log::trace!("HyphenSeparatedParticleAnalyzer::parse()");
        #[cfg(feature = "trace_analyzer")]
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        let morph = ctx.morph();

        for &particle in self.particles_after_hyphen {
            let word_lower = ctx.word_lower();
            if word_lower.len() <= particle.len() || !word_lower.ends_with(particle) {
                continue;
            };
            #[cfg(feature = "trace_analyzer")]
            log::trace!(r#" particle: "{}" "#, particle);
            let unsuffixed_word = &word_lower[..word_lower.len() - particle.len()];
            #[cfg(feature = "trace_analyzer")]
            log::trace!(r#" unsuffixed_word: "{}" "#, unsuffixed_word);

            'subparse: for parsed in morph.parse(unsuffixed_word) {
                #[cfg(feature = "trace_analyzer")]
                log::trace!(r#" subparsed: {:?} "#, parsed);
                let lex_stack = parsed.lex.stack;
                // If a word ends with with one of the particles, it can't ends with an another.
                if lex_stack.particle.is_some() {
                    continue 'subparse;
                };
                let h_stack = lex_stack.stack.clone();
                let hs_particle = HyphenSeparatedParticle::new(particle);
                let stack = StackParticle::new(h_stack, hs_particle);
                let lex = Lex::from_stack(stack);
                let score = parsed.score * self.score_multiplier;
                ctx.push(Parsed::new(lex, score));
            }
        }
    }
}
