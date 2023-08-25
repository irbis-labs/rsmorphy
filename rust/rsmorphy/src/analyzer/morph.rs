use std::path::Path;
use std::sync::Arc;

use crate::analyzer::context::ParseContext;
use crate::analyzer::model::ParseResult;
use crate::analyzer::unit::AnalyzerUnit;
use crate::analyzer::units::*;
use crate::estimator::SingleTagProbabilityEstimator;
use crate::opencorpora::dictionary::Dictionary;

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
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    dict: Dictionary,
    estimator: SingleTagProbabilityEstimator,
    units: Units,
}

impl MorphAnalyzer {
    /// Creates `MorphAnalyzer` with preloaded dict
    pub fn new(dict: Dictionary) -> Self {
        let estimator = SingleTagProbabilityEstimator {};
        let units = Units::default();
        let inner = Arc::new(Inner {
            dict,
            estimator,
            units,
        });
        MorphAnalyzer { inner }
    }

    /// Loads `Dictionary` from disk and creates `MorphAnalyzer`
    pub fn from_file<P>(p: P) -> Self
    where
        P: AsRef<Path>,
    {
        let dict = Dictionary::from_file(p);
        MorphAnalyzer::new(dict)
    }

    #[inline]
    pub fn dict(&self) -> &Dictionary {
        &self.inner.dict
    }

    #[inline]
    pub fn units(&self) -> &Units {
        &self.inner.units
    }

    /// Analyze the word and return a list of `Parsed`:
    pub fn parse(&self, word: &str) -> ParseResult {
        let mut context = ParseContext::new(self, word);

        self.look_over(&mut context);
        context.apply_estimator(&self.inner.estimator);
        context.finish()
    }

    fn look_over(&self, ctx: &mut ParseContext) {
        macro_rules! look_in (
                ($t: ident) => {{
                    self.units().$t.parse(ctx);
                }};
                ($t: ident, stop_if_found) => {{
                    self.units().$t.parse(ctx);
                    if !ctx.result().is_empty() { return };
                }}
            );

        look_in!(dictionary);
        look_in!(initials, stop_if_found);

        look_in!(number, stop_if_found);

        look_in!(punct, stop_if_found);

        look_in!(roman);
        look_in!(latin, stop_if_found);

        look_in!(hsp, stop_if_found);

        look_in!(ha, stop_if_found);

        look_in!(hword, stop_if_found);

        look_in!(kp, stop_if_found);

        look_in!(up);
        look_in!(ks, stop_if_found);

        look_in!(unknown, stop_if_found);

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use crate::util::test::init_logger;
    use crate::util::test::RU;

    #[test]
    fn load_ru() {
        init_logger();
        let _ = RU.dict();
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
