use inlinable_string::InlinableString;
use inlinable_string::StringExt;

use crate::analyzer::common::ParseContext;
use crate::analyzer::common::ParseVec;
use crate::analyzer::common::SeenSet;
use crate::analyzer::Lang;

#[derive(Debug)]
pub struct ParseBuffers<'m, L: Lang> {
    pub(super) morph: &'m L,
    pub(super) parses: ParseVec,
    pub(super) seen_parses: SeenSet<'m>,
    pub(super) word_lower: InlinableString,
}

impl<'m, L: Lang> ParseBuffers<'m, L> {
    #[inline(always)]
    pub fn new(morph: &'m L) -> Self {
        let parses = ParseVec::new();
        let seen_parses = SeenSet::new();
        let word_lower = InlinableString::new();
        ParseBuffers {
            morph,
            parses,
            seen_parses,
            word_lower,
        }
    }

    #[inline(always)]
    pub fn morph(&self) -> &'m L {
        self.morph
    }

    pub fn prepare<'w, 'b>(&'b mut self, word: &'w str) -> ParseContext<'m, 'w, 'b, L> {
        debug_assert!(self.parses.is_empty());
        debug_assert!(self.seen_parses.is_empty());
        debug_assert!(self.word_lower.is_empty());

        self.word_lower = word.chars().flat_map(char::to_lowercase).collect();

        ParseContext::new(
            self.morph,
            word,
            &mut self.word_lower,
            &mut self.parses,
            &mut self.seen_parses,
        )
    }

    #[inline(always)]
    pub fn parse<'w, 'b>(&'b mut self, word: &'w str) -> ParseContext<'m, 'w, 'b, L> {
        self.prepare(word).parse().estimate()
    }
}
