use inlinable_string::InlinableString;

use crate::analyzer::common::parse::Parse;
use crate::analyzer::common::parse::ParseResult;
use crate::analyzer::common::ParseVec;
use crate::analyzer::common::ProbabilityEstimator;
use crate::analyzer::common::Seen;
use crate::analyzer::common::SeenSet;
use crate::analyzer::Lang;

#[derive(Debug)]
pub struct ParseContext<'m, 's, L: Lang> {
    morph: &'m L,
    parses: ParseVec,
    word: &'s str,
    word_lower: InlinableString,
    seen_parses: SeenSet<'m>,
}

impl<'m, 's, L: Lang> ParseContext<'m, 's, L> {
    pub fn new(morph: &'m L, word: &'s str) -> Self {
        let word_lower = word.chars().flat_map(char::to_lowercase).collect();
        let parses = ParseVec::new();
        let seen_parses = SeenSet::new();
        ParseContext {
            morph,
            parses,
            word,
            word_lower,
            seen_parses,
        }
    }

    #[inline(always)]
    pub fn finish(self) -> ParseResult<L> {
        ParseResult::new(self.morph.clone(), self.parses.into_inner())
    }

    #[inline]
    pub fn morph(&self) -> &'m L {
        self.morph
    }

    #[inline]
    pub fn parses(&self) -> &ParseVec {
        &self.parses
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
    pub fn push(&mut self, parse: Parse) {
        self.parses.vec.push(parse);
    }

    #[inline]
    pub fn push_if_not_seen(&mut self, parse: Parse) {
        let seen = self.morph.make_seen(&parse.lex);
        if self.seen_parses.insert(seen) {
            self.push(parse);
        }
    }

    #[inline]
    pub fn mark_as_seen(&mut self, seen: Seen<'m>) -> bool {
        self.seen_parses.insert(seen)
    }

    #[inline]
    pub fn extend(&mut self, parses: impl IntoIterator<Item = Parse>) {
        self.parses.vec.extend(parses);
    }

    #[inline]
    pub fn apply_estimator(&mut self, estimator: impl ProbabilityEstimator) {
        estimator.apply_to_parses(
            self.morph,
            self.word,
            &self.word_lower,
            &mut self.parses.vec,
        );
    }
}
