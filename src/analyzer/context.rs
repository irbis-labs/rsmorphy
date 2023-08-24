use inlinable_string::InlinableString;

use crate::analyzer::model::Seen;
use crate::analyzer::model::SeenSet;
use crate::estimator::ProbabilityEstimator;
use crate::MorphAnalyzer;
use crate::ParseResult;
use crate::Parsed;

#[derive(Debug)]
pub struct ParseContext<'m, 's> {
    morph: &'m MorphAnalyzer,
    result: ParseResult,
    word: &'s str,
    word_lower: InlinableString,
    seen_parses: SeenSet<'m>,
}

impl<'m, 's> ParseContext<'m, 's> {
    pub fn new(morph: &'m MorphAnalyzer, word: &'s str) -> Self {
        // FIXME construct word_lower without allocation.
        let word_lower = word.to_lowercase().into();
        let result = ParseResult::new();
        let seen_parses = SeenSet::default();
        ParseContext {
            morph,
            result,
            word,
            word_lower,
            seen_parses,
        }
    }

    pub fn finish(self) -> ParseResult {
        self.result
    }

    #[inline]
    pub fn morph(&self) -> &'m MorphAnalyzer {
        self.morph
    }

    #[inline]
    pub fn result(&self) -> &ParseResult {
        &self.result
    }

    #[inline]
    pub fn word(&self) -> &'s str {
        self.word
    }

    #[inline]
    pub fn word_lower(&self) -> &str {
        &self.word_lower
    }

    #[inline]
    pub fn seen_parses(&self) -> &SeenSet {
        &self.seen_parses
    }

    #[inline]
    pub fn push(&mut self, variant: Parsed) {
        self.result.list.push(variant);
    }

    #[inline]
    pub fn push_if_not_seen(&mut self, variant: Parsed) {
        let morph = self.morph;
        if self.seen_parses.insert(variant.lex.to_seen(morph)) {
            self.push(variant);
        }
    }

    #[inline]
    pub fn mark_as_seen(&mut self, seen: Seen<'m>) -> bool {
        self.seen_parses.insert(seen)
    }

    #[inline]
    pub fn extend(&mut self, variants: impl IntoIterator<Item = Parsed>) {
        self.result.list.extend(variants);
    }

    #[inline]
    pub fn apply_estimator(&mut self, estimator: impl ProbabilityEstimator) {
        estimator.apply_to_parses(
            self.morph,
            self.word,
            &self.word_lower,
            &mut self.result.list,
        );
    }
}
