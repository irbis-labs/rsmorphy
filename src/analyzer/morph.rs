use std::path::Path;

use container::{/*Parsed, */ParseResult, SeenSet};
use opencorpora::dictionary::Dictionary;

use analyzer::units::*;
use estimator::SingleTagProbabilityEstimator;


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


#[derive(Debug, Clone)]
pub struct MorphAnalyzer {
    pub dict: Dictionary,
    pub prob_estimator: SingleTagProbabilityEstimator,
    pub units: Units,
}


impl MorphAnalyzer {
    pub fn from_file<P>(p: P) -> Self where P: AsRef<Path> {

        let dictionary = Dictionary::from_file(p);
        // char_substitutes = dictionary.words.compile_replaces(char_substitutes or {})

        MorphAnalyzer {
            dict: dictionary,
            prob_estimator: SingleTagProbabilityEstimator {},
            units: Units::default()
        }
    }

    /// Analyze the word and return a list of :class:`pymorphy2.analyzer.Parse`
    /// namedtuples:
    ///
    /// Parse(word, tag, normal_form, para_id, idx, _score)
    ///
    /// (or plain tuples if ``result_type=None`` was used in constructor).
    ///
    pub fn parse(&self, word: &str) -> ParseResult {
        let word_lower = word.to_lowercase();
        let mut result: ParseResult = ParseResult::new();
        let mut seen: SeenSet = SeenSet::default();

        'analyze: loop {
            macro_rules! analyze (
                ($t: ident, $is_terminal: expr) => {{
                    self.units.$t.parse(self, &mut result, word, &word_lower, &mut seen);
                    if $is_terminal && !result.is_empty() { break 'analyze };
                }}
            );

            {
                analyze!(dictionary, false);
                analyze!(initials, true);
            }
            analyze!(number, true);
            analyze!(punct, true);
            {
                analyze!(roman, false);
                analyze!(latin, true);
            }
            analyze!(hsp, true);
            analyze!(ha, true);
            analyze!(hword, true);
            analyze!(kp, true);
            {
                analyze!(up, false);
                analyze!(ks, true);
            }
            analyze!(unknown, true);

            unreachable!();
        }

        self.prob_estimator.apply_to_parses(self, word, &word_lower, &mut result);
        result
    }
}


//#[cfg(test)]
//mod tests {
//    use std::path::Path;
//    use super::*;
//
//    #[test]
//    #[ignore]
//    fn load() {
//        let morph = MorphAnalyzer::from_file(Path::new("./assets/pymorphy2-dicts-ru"));
//    }
//
//    #[test]
//    #[ignore]
//    fn load_ru() {
//        let morph = MorphAnalyzer::from_file(Path::new("./assets/pymorphy2-dicts-ru-2.4.393658.3725883"));
//    }
//
//    #[test]
//    #[ignore]
//    fn load_uk() {
//        let morph = MorphAnalyzer::from_file(Path::new("./assets/pymorphy2-dicts-uk-2.4.1.1.1460299261"));
//    }
//
//    #[test]
//    #[ignore]
//    fn parse() {
//        let morph = ::load_test_morph_ru();
//        morph.parse("менимальный");
//
//    }
//}
