use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use inlinable_string::InlinableString;

use crate::opencorpora::{OpencorporaTagReg, dictionary::ParadigmId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seen<'a> {
    pub word: InlinableString,
    pub tag: &'a OpencorporaTagReg,
    pub para_id: Option<ParadigmId>,
}

impl<'a> Seen<'a> {
    pub fn new<W, P, PID>(word: W, tag: &'a OpencorporaTagReg, para_id: P) -> Self
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

/// A thin hash set
#[derive(Default, Debug, Clone)]
pub struct SeenSet {
    vec: Vec<u64>,
}

impl SeenSet {
    pub fn insert(&mut self, s: &Seen) -> bool {
        let hash = Self::eval_hash(s);
        if self.vec.contains(&hash) {
            false
        } else {
            self.vec.push(hash);
            true
        }
    }

    pub fn contains(&self, s: &Seen) -> bool {
        self.vec.contains(&Self::eval_hash(s))
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    fn eval_hash(s: &Seen) -> u64 {
        let mut hasher = DefaultHasher::default();
        s.hash(&mut hasher);
        hasher.finish()
    }
}
