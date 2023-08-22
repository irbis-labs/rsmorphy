use std::ops;

use smallvec::SmallVec;

use crate::Parsed;

type ParsedList = SmallVec<[Parsed; 8]>;

#[derive(Debug, Default, Clone)]
pub struct ParseResult {
    pub(crate) list: ParsedList
}

impl ops::Deref for ParseResult {
    type Target = [Parsed];

    fn deref(&self) -> &Self::Target {
        &*self.list
    }
}

impl ops::DerefMut for ParseResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

impl ParseResult {
    pub fn new() -> Self {
        ParseResult::default()
    }

    pub fn into_inner(self) -> ParsedList {
        self.list
    }
}

impl IntoIterator for ParseResult {
    type Item = Parsed;
    type IntoIter = <ParsedList as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}
