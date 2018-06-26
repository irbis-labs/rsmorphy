use analyzer::MorphAnalyzer;
use analyzer::units::abc::*;
use container::HyphenSeparatedParticle;
use container::{Parsed, ParseResult, SeenSet};
use container::Lex;
use container::stack::StackParticle;


// TODO move into `Dictionary`
pub static PARTICLES_AFTER_HYPHEN: [&'static str; 8] = [
    "-то", "-ка", "-таки", "-де", "-тко", "-тка", "-с", "-ста",
];

pub const SCORE_DECAY: f64 = 0.9;


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

#[derive(Default, Debug, Clone)]
pub struct HyphenSeparatedParticleAnalyzer {}

impl AnalyzerUnit for HyphenSeparatedParticleAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("HyphenSeparatedParticleAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        for &particle in &PARTICLES_AFTER_HYPHEN {
            if word_lower.len() <= particle.len() || !word_lower.ends_with(particle) {
                continue
            };
            trace!(r#" particle: "{}" "#, particle);
            let unsuffixed_word = &word_lower[.. word_lower.len() - particle.len()];
            trace!(r#" unsuffixed_word: "{}" "#, unsuffixed_word);

            'subparse: for parsed in morph.parse(unsuffixed_word) {
                trace!(r#" subparsed: {:?} "#, parsed);
                let lex_stack = parsed.lex.stack;
                // If a word ends with with one of the particles, it can't ends with an another.
                if lex_stack.particle.is_some() { continue 'subparse };
                let h_stack = lex_stack.stack.clone();
                let hs_particle = HyphenSeparatedParticle::new(particle);
                let container = StackParticle::new(h_stack, hs_particle);
                let lex = Lex::from_stack(morph, container);
                let score = parsed.score * SCORE_DECAY;
                result.push(Parsed::new(lex, score));
            }
        }
    }
}
