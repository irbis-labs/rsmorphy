use smallvec::SmallVec;

use crate::analyzer::common::parse::Parse;
use crate::analyzer::Lang;

#[derive(Debug, Clone)]
pub struct ParseResult<L: Lang> {
    pub(in crate::analyzer) morph: L,
    pub(in crate::analyzer) vec: SmallVec<[Parse; 32]>,
}

impl<L: Lang> ParseResult<L> {
    #[inline(always)]
    pub fn new(morph: L, vec: SmallVec<[Parse; 32]>) -> Self {
        ParseResult { morph, vec }
    }

    #[inline(always)]
    pub fn morph(&self) -> &L {
        &self.morph
    }

    #[inline(always)]
    pub fn vec(&self) -> &SmallVec<[Parse; 32]> {
        &self.vec
    }
}

impl<L: Lang> IntoIterator for ParseResult<L> {
    type Item = Parse;
    type IntoIter = smallvec::IntoIter<[Parse; 32]>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}
