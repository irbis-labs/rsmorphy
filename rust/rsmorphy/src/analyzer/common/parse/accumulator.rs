use std::mem;
use std::ops;

use smallvec::SmallVec;

use crate::analyzer::common::parse::Parse;

#[derive(Debug, Clone)]
pub struct ParseVec {
    pub(in crate::analyzer) vec: SmallVec<[Parse; 32]>,
}

impl ParseVec {
    #[inline(always)]
    pub fn new() -> Self {
        ParseVec {
            vec: SmallVec::new(),
        }
    }

    #[inline(always)]
    pub fn take(&mut self) -> SmallVec<[Parse; 32]> {
        mem::replace(&mut self.vec, SmallVec::new())
    }

    pub fn drain(&mut self, range: ops::RangeFull) -> impl Iterator<Item = Parse> + '_ {
        self.vec.drain(range)
    }

    #[inline(always)]
    pub fn into_inner(self) -> SmallVec<[Parse; 32]> {
        self.vec
    }
}

impl ops::Deref for ParseVec {
    type Target = [Parse];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl ops::DerefMut for ParseVec {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
