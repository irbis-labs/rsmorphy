use std::collections::HashSet;
use std::hash::Hash;
use std::ops;

use inlinable_string::InlinableString;

use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::tag::TagReg;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seen<'a> {
    pub word: InlinableString,
    pub tag: &'a TagReg,
    pub para_id: Option<ParadigmId>,
}

impl<'a> Seen<'a> {
    pub fn new<W, P, PID>(word: W, tag: &'a TagReg, para_id: P) -> Self
    where
        W: Into<InlinableString>,
        P: Into<Option<PID>>,
        PID: Into<ParadigmId>,
    {
        let word = word.into();
        let para_id = para_id.into().map(Into::into);
        Seen { word, tag, para_id }
    }
}

// TODO Replace with a SmallVec-like alternative for HashSet.
#[derive(Default, Debug, Clone)]
pub struct SeenSet<'a> {
    set: HashSet<Seen<'a>>,
}

impl<'a> SeenSet<'a> {
    pub fn new() -> Self {
        SeenSet {
            set: HashSet::new(),
        }
    }
}

impl<'a> ops::Deref for SeenSet<'a> {
    type Target = HashSet<Seen<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

impl<'a> ops::DerefMut for SeenSet<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.set
    }
}
