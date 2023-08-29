pub mod abbr;
pub mod by_analogy;
pub mod by_hyphen;
pub mod by_lookup;
pub mod by_shape;

pub mod unknown;

pub use self::abbr::initial::InitialsAnalyzer;
pub use self::by_analogy::kp::KnownPrefixAnalyzer;
pub use self::by_analogy::ks::KnownSuffixAnalyzer;
pub use self::by_analogy::up::UnknownPrefixAnalyzer;
pub use self::by_hyphen::adverb::HyphenAdverbAnalyzer;
pub use self::by_hyphen::separate_particle::HyphenSeparatedParticleAnalyzer;
pub use self::by_hyphen::words::HyphenatedWordsAnalyzer;
pub use self::by_lookup::dict::DictionaryAnalyzer;
pub use self::by_shape::latin::LatinAnalyzer;
pub use self::by_shape::number::NumberAnalyzer;
pub use self::by_shape::punct::PunctuationAnalyzer;
pub use self::by_shape::roman::RomanAnalyzer;
pub use self::unknown::UnknownAnalyzer;
use crate::opencorpora::gram::GramRegistry;

#[derive(Debug, Clone)]
pub struct Units {
    pub dictionary: DictionaryAnalyzer,
    pub initial: InitialsAnalyzer,
    pub latin: LatinAnalyzer,
    pub number: NumberAnalyzer,
    pub roman: RomanAnalyzer,
    pub punct: PunctuationAnalyzer,
    pub ha: HyphenAdverbAnalyzer,
    pub hsp: HyphenSeparatedParticleAnalyzer,
    pub hword: HyphenatedWordsAnalyzer,
    pub kp: KnownPrefixAnalyzer,
    pub ks: KnownSuffixAnalyzer,
    pub up: UnknownPrefixAnalyzer,
    pub unknown: UnknownAnalyzer,
}

impl Units {
    pub fn new(grammemes: &GramRegistry) -> Self {
        Units {
            dictionary: DictionaryAnalyzer::new(),
            initial: InitialsAnalyzer::new(grammemes),
            latin: LatinAnalyzer::new(grammemes),
            number: NumberAnalyzer::new(grammemes),
            roman: RomanAnalyzer::new(grammemes),
            punct: PunctuationAnalyzer::new(grammemes),
            ha: HyphenAdverbAnalyzer::new(grammemes),
            hsp: HyphenSeparatedParticleAnalyzer::new(),
            hword: HyphenatedWordsAnalyzer::new(),
            kp: KnownPrefixAnalyzer::new(),
            ks: KnownSuffixAnalyzer::new(),
            up: UnknownPrefixAnalyzer::new(),
            unknown: UnknownAnalyzer::new(grammemes),
        }
    }
}
