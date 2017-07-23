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


#[cfg(test)]
mod tests {
    use test::black_box;
    use super::*;
    use ::load_test_morph_ru;
    // use ::load_test_morph_uk;


    lazy_static!{
        static ref RU: MorphAnalyzer = load_test_morph_ru();
        // static ref UK: MorphAnalyzer = load_test_morph_uk();
    }


    #[test]
    fn load_ru() {
        black_box(&RU);
    }

    // FIXME ukrainian
    // #[test]
    // fn load_uk() {
    //     black_box(&UK);
    // }

    #[test]
    fn parse() {
        assert_eq!(RU.parse("менимальный").len(), 3);
    }
}
