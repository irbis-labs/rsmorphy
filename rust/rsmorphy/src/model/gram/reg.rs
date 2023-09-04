use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;

use inlinable_string::InlinableString;

use super::Gram;
use super::GramId;
use crate::model::gram::GramSet;
use crate::model::tag::TagSet;

#[derive(Debug, Clone)]
pub struct GramReg {
    pub gram: Gram,
    pub parent: Option<Gram>,
    pub name: GramId,
    pub alias: InlinableString,
    pub description: InlinableString,
    pub children: GramSet,
    pub incompatible: GramSet,
    pub tag: TagSet,
    pub extra: bool,
}

impl PartialEq<Self> for GramReg {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.gram == other.gram
    }
}

impl Eq for GramReg {}

impl PartialOrd for GramReg {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.gram.partial_cmp(&other.gram)
    }
}

impl Ord for GramReg {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.gram.cmp(&other.gram)
    }
}

impl Hash for GramReg {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.gram.hash(state)
    }
}
