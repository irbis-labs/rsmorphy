use crate::analyzer::common::util::Prefixes;
use crate::analyzer::common::DictionaryAnalyzer;
use crate::analyzer::common::HyphenAdverbAnalyzer;
use crate::analyzer::common::HyphenSeparatedParticleAnalyzer;
use crate::analyzer::common::HyphenatedWordsAnalyzer;
use crate::analyzer::common::InitialsAnalyzer;
use crate::analyzer::common::KnownPrefixAnalyzer;
use crate::analyzer::common::KnownSuffixAnalyzer;
use crate::analyzer::common::LatinAnalyzer;
use crate::analyzer::common::NumberAnalyzer;
use crate::analyzer::common::PunctuationAnalyzer;
use crate::analyzer::common::RomanAnalyzer;
use crate::analyzer::common::UnknownAnalyzer;
use crate::analyzer::common::UnknownPrefixAnalyzer;
use crate::analyzer::ru::config::INITIAL_LETTERS;
use crate::analyzer::ru::config::KNOWN_PREFIXES;
use crate::analyzer::ru::config::PARTICLES_AFTER_HYPHEN;
use crate::analyzer::Russian;
use crate::model::gram::GramRegistry;

#[derive(Debug)]
pub(super) struct Units {
    pub dictionary: DictionaryAnalyzer<Russian>,
    pub initial: InitialsAnalyzer<Russian>,
    pub latin: LatinAnalyzer<Russian>,
    pub number: NumberAnalyzer<Russian>,
    pub roman: RomanAnalyzer<Russian>,
    pub punct: PunctuationAnalyzer<Russian>,
    pub ha: HyphenAdverbAnalyzer<Russian>,
    pub hsp: HyphenSeparatedParticleAnalyzer<Russian>,
    pub hword: HyphenatedWordsAnalyzer<Russian>,
    pub kp: KnownPrefixAnalyzer<Russian>,
    pub ks: KnownSuffixAnalyzer<Russian>,
    pub up: UnknownPrefixAnalyzer<Russian>,
    pub unknown: UnknownAnalyzer<Russian>,
}

impl Units {
    pub fn new(grammemes: &GramRegistry) -> Self {
        let known_prefixes = Prefixes::new(KNOWN_PREFIXES);
        Units {
            dictionary: DictionaryAnalyzer::new(None),
            initial: InitialsAnalyzer::new(grammemes, INITIAL_LETTERS, None),
            latin: LatinAnalyzer::new(grammemes, None),
            number: NumberAnalyzer::new(grammemes, None),
            roman: RomanAnalyzer::new(grammemes, None),
            punct: PunctuationAnalyzer::new(grammemes, None),
            ha: HyphenAdverbAnalyzer::new(grammemes, None),
            hsp: HyphenSeparatedParticleAnalyzer::new(PARTICLES_AFTER_HYPHEN, None),
            hword: HyphenatedWordsAnalyzer::new(known_prefixes.clone(), None),
            kp: KnownPrefixAnalyzer::new(known_prefixes, None, None),
            ks: KnownSuffixAnalyzer::new(None, None),
            up: UnknownPrefixAnalyzer::new(None, None, None, None),
            unknown: UnknownAnalyzer::new(grammemes, None),
        }
    }
}
