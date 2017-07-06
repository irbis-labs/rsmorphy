use analyzer::MorphAnalyzer;
use analyzer::units::abc::*;
use container::HyphenSeparatedParticle;
use container::{Parsed, ParseResult, SeenSet};
use container::Lex;
use container::stack::Stack;
use container::stack::StackHyphenated;
use container::stack::StackParticle;


pub static PARTICLES_AFTER_HYPHEN: [&'static str; 8] = [
    "-то", "-ка", "-таки", "-де", "-тко", "-тка", "-с", "-ста",
];


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
pub struct HyphenSeparatedParticleAnalyzer {

}


impl Analyzer for HyphenSeparatedParticleAnalyzer {
    fn parse(&self, morph: &MorphAnalyzer, result: &mut ParseResult, word: &str, word_lower: &str, _seen_parses: &mut SeenSet) {
        trace!("HyphenSeparatedParticleAnalyzer::parse()");
        trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

        for particle in &PARTICLES_AFTER_HYPHEN {
            if word_lower.len() <= particle.len() || !word_lower.ends_with(particle) {
                continue
            };
            trace!(r#" particle: "{}" "#, particle);
            let unsuffixed_word = &word_lower[.. word_lower.len() - particle.len()];
            trace!(r#" unsuffixed_word: "{}" "#, unsuffixed_word);

            'subparse: for parsed in morph.parse(unsuffixed_word) {
                trace!(r#" subparsed: {:?} "#, parsed);
                let stack: StackHyphenated = match parsed.lex.stack {
                    // If a word ends with with one of the particles, it can't ends with an another.
                    Stack::HSP(_)               => continue 'subparse,
                    Stack::Hyphenated(ref v)    => v.clone(),
                    Stack::Affix(ref v)         => v.clone().into(),
                    Stack::Source(ref v)        => v.clone().into(),
                };
                let container = StackParticle {
                    stack: stack,
                    particle: Some(HyphenSeparatedParticle {
                        particle: particle.to_string(),
                    })
                };
                result.push(Parsed {
                    lex: Lex::from_stack(morph, container),
                    score: parsed.score * 0.9,
                });
            }
        }
    }
}
