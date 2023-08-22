use std::path::Path;

use crate::{
    analyzer::units::*,
    container::{ParseResult, SeenSet},
    estimator::SingleTagProbabilityEstimator,
    opencorpora::dictionary::Dictionary,
};

#[derive(Debug, Default, Clone)]
pub struct Units {
    pub dictionary: DictionaryAnalyzer,
    pub initials: InitialsAnalyzer,
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

#[derive(Debug)]
pub struct MorphAnalyzer {
    pub dict: Dictionary,
    pub estimator: SingleTagProbabilityEstimator,
    pub units: Units,
}

impl MorphAnalyzer {
    /// Creates `MorphAnalyzer` with preloaded dict
    pub fn new(dict: Dictionary) -> Self {
        let estimator = SingleTagProbabilityEstimator {};
        let units = Units::default();
        MorphAnalyzer {
            dict,
            estimator,
            units,
        }
    }

    /// Loads `Dictionary` from disk and creates `MorphAnalyzer`
    pub fn from_file<P>(p: P) -> Self
    where
        P: AsRef<Path>,
    {
        let dict = Dictionary::from_file(p);
        MorphAnalyzer::new(dict)
    }

    /// Analyze the word and return a list of `Parsed`:
    pub fn parse(&self, word: &str) -> ParseResult {
        let word_lower = word.to_lowercase();

        let look_over = || -> ParseResult {
            let mut result = ParseResult::new();
            let mut seen = SeenSet::default();

            macro_rules! look_in (
                ($t: ident) => {{
                    self.units.$t.parse(self, &mut result, word, &word_lower, &mut seen);
                }};
                ($t: ident, return) => {{
                    self.units.$t.parse(self, &mut result, word, &word_lower, &mut seen);
                    if !result.is_empty() { return result };
                }}
            );

            look_in!(dictionary);
            look_in!(initials, return);

            look_in!(number, return);

            look_in!(punct, return);

            look_in!(roman);
            look_in!(latin, return);

            look_in!(hsp, return);

            look_in!(ha, return);

            look_in!(hword, return);

            look_in!(kp, return);

            look_in!(up);
            look_in!(ks, return);

            look_in!(unknown, return);

            unreachable!();
        };

        let mut result = look_over();
        self.estimator
            .apply_to_parses(self, word, &word_lower, &mut result.list);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::util::test::{RU, init_logger};

    #[test]
    fn load_ru() {
        init_logger();
        let _ = RU.dict;
    }

    #[test]
    fn parse() {
        init_logger();
        assert_eq!(RU.parse("минимальный").len(), 2);
        assert_eq!(RU.parse("менимальный").len(), 3);
    }

    #[test]
    fn parse_one_letter() {
        init_logger();
        assert_eq!(RU.parse("с").len(), 25);
    }

    #[test]
    fn parse_one_letter_and_dot() {
        init_logger();
        assert_eq!(RU.parse("м.").len(), 1);
    }

    #[test]
    fn parse_unknown_two_letter() {
        init_logger();
        assert_eq!(RU.parse("ТМ").len(), 1);
        assert_eq!(RU.parse("КТ").len(), 1);
        assert_eq!(RU.parse("1С").len(), 1);
    }

    #[test]
    fn parse_dash() {
        init_logger();
        assert_eq!(RU.parse("Р-ка").len(), 1);
        assert_eq!(RU.parse("з-то").len(), 1);
    }
}
