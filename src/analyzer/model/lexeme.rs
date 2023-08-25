use std::iter::FromIterator;
use std::ops;

use derive_more::From;
use smallvec::SmallVec;

use crate::Lex;

type LexList = SmallVec<[Lex; 32]>;

#[derive(Debug, Clone, From, PartialEq)]
pub struct Lexeme {
    pub(in crate::analyzer) list: LexList,
}

impl ops::Deref for Lexeme {
    type Target = [Lex];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &*self.list
    }
}

impl Lexeme {
    #[inline(always)]
    pub fn into_inner(self) -> LexList {
        self.list
    }
}

impl IntoIterator for Lexeme {
    type Item = Lex;
    type IntoIter = <LexList as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

impl FromIterator<Lex> for Lexeme {
    #[inline(always)]
    fn from_iter<T: IntoIterator<Item = Lex>>(iter: T) -> Self {
        let list = iter.into_iter().collect();
        Lexeme { list }
    }
}
