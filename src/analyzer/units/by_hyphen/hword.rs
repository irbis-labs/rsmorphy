use crate::{
    analyzer::{units::abc::*, MorphAnalyzer},
    container::{
        ParseResult,
        SeenSet,
        //        Dictionary,
        //        HyphenSeparatedParticle,
        //        Lex,
        //        Score,
        //        stack::{Stack, StackAffix, StackHyphenated},
    },
};

/// Parse the word by parsing its hyphen-separated parts.
///
/// Examples:
///
/// * интернет-магазин -> "интернет-" + магазин
/// * человек-гора -> человек + гора

#[derive(Default, Debug, Clone, Copy)]
pub struct HyphenatedWordsAnalyzer {}

impl AnalyzerUnit for HyphenatedWordsAnalyzer {
    fn parse(
        &self,
        _morph: &MorphAnalyzer,
        _result: &mut ParseResult,
        word: &str,
        word_lower: &str,
        _seen_parses: &mut SeenSet,
    ) {
        log::trace!("HyphenatedWordsAnalyzer::parse()");
        log::trace!(r#" word = "{}", word_lower = "{}" "#, word, word_lower);

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
