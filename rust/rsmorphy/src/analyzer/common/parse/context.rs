use std::fmt;

use inlinable_string::InlinableString;
use inlinable_string::StringExt;

use crate::analyzer::common::parse::Parse;
use crate::analyzer::common::parse::ParseResult;
use crate::analyzer::common::ParseVec;
use crate::analyzer::common::ProbabilityEstimator;
use crate::analyzer::common::Seen;
use crate::analyzer::common::SeenSet;
use crate::analyzer::Lang;

pub struct ParseContext<'m, 'w, 'b, L: Lang> {
    morph: &'m L,
    word: &'w str,
    word_lower: &'b mut InlinableString,
    parses: &'b mut ParseVec,
    seen_parses: &'b mut SeenSet<'m>,
}

impl<'m, 'w, 'b, L: Lang> fmt::Debug for ParseContext<'m, 'w, 'b, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParseContext")
            .field("morph", &"...")
            .field("word", &self.word)
            .field("word_lower", &self.word_lower)
            .field("parses", &self.parses)
            .field("seen_parses", &self.seen_parses)
            .finish()
    }
}

impl<'m, 'w, 'b, L: Lang> ParseContext<'m, 'w, 'b, L> {
    #[inline(always)]
    pub(super) fn new(
        morph: &'m L,
        word: &'w str,
        word_lower: &'b mut InlinableString,
        parses: &'b mut ParseVec,
        seen_parses: &'b mut SeenSet<'m>,
    ) -> Self {
        ParseContext {
            morph,
            word,
            word_lower,
            parses,
            seen_parses,
        }
    }

    #[inline(always)]
    pub fn parse(mut self) -> Self {
        self.morph.parse_prepared(&mut self);
        self
    }

    #[inline(always)]
    pub fn estimate(mut self) -> Self {
        self.morph.estimate_prepared(&mut self);
        self
    }

    #[inline(always)]
    pub fn iter<'s>(&'s self) -> impl Iterator<Item = &'s Parse> + 's {
        self.parses.iter()
    }

    pub fn drain<'s>(&'s mut self) -> impl Iterator<Item = Parse> + 's {
        self.word_lower.clear();
        self.seen_parses.clear();
        self.parses.drain(..)
    }

    pub fn take_result(self) -> ParseResult<L> {
        self.word_lower.clear();
        self.seen_parses.clear();
        ParseResult::new(self.morph.clone(), self.parses.take())
    }

    #[inline(always)]
    pub fn into_iter(self) -> impl Iterator<Item = Parse> {
        self.parses.take().into_iter()
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
    pub fn word(&self) -> &'w str {
        self.word
    }

    #[inline]
    pub fn word_lower<'s: 'b>(&'s self) -> &'b InlinableString {
        self.word_lower
    }

    #[inline]
    pub fn seen_parses(&self) -> &'b SeenSet {
        self.seen_parses
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

impl<'m, 'w, 'b, L: Lang> Drop for ParseContext<'m, 'w, 'b, L> {
    fn drop(&mut self) {
        self.word_lower.clear();
        self.seen_parses.clear();
        self.parses.vec.clear();
    }
}
