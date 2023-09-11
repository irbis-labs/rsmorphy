use crate::analyzer::common::DictionaryAnalyzer;
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
use crate::analyzer::uk::config::KNOWN_PREFIXES;
use crate::analyzer::uk::config::PARTICLES_AFTER_HYPHEN;
use crate::analyzer::Ukrainian;
use crate::data_struct::prefixes::Prefixes;
use crate::model::gram::GramRegistry;

#[derive(Debug)]
pub(super) struct Units {
    pub dictionary: DictionaryAnalyzer<Ukrainian>,
    pub initial: InitialsAnalyzer<Ukrainian>,
    pub latin: LatinAnalyzer<Ukrainian>,
    pub number: NumberAnalyzer<Ukrainian>,
    pub roman: RomanAnalyzer<Ukrainian>,
    pub punct: PunctuationAnalyzer<Ukrainian>,
    pub hsp: HyphenSeparatedParticleAnalyzer<Ukrainian>,
    pub hword: HyphenatedWordsAnalyzer<Ukrainian>,
    pub kp: KnownPrefixAnalyzer<Ukrainian>,
    pub ks: KnownSuffixAnalyzer<Ukrainian>,
    pub up: UnknownPrefixAnalyzer<Ukrainian>,
    pub unknown: UnknownAnalyzer<Ukrainian>,
}

impl Units {
    pub fn new(grammemes: GramRegistry<'_>) -> Self {
        let known_prefixes = Prefixes::new(KNOWN_PREFIXES);
        Units {
            dictionary: DictionaryAnalyzer::new(None),
            initial: InitialsAnalyzer::new(grammemes, None),
            latin: LatinAnalyzer::new(grammemes, None),
            number: NumberAnalyzer::new(grammemes, None),
            roman: RomanAnalyzer::new(grammemes, None),
            punct: PunctuationAnalyzer::new(grammemes, None),
            hsp: HyphenSeparatedParticleAnalyzer::new(PARTICLES_AFTER_HYPHEN, None),
            hword: HyphenatedWordsAnalyzer::new(known_prefixes.clone(), None),
            kp: KnownPrefixAnalyzer::new(known_prefixes, None, None),
            ks: KnownSuffixAnalyzer::new(None, None),
            up: UnknownPrefixAnalyzer::new(None, None, None, None),
            unknown: UnknownAnalyzer::new(grammemes, None),
        }
    }
}
