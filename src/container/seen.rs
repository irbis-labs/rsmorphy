use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


use ::opencorpora::tag::OpencorporaTagReg;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seen<'a> {
    pub word: Cow<'a, str>,
    pub tag: &'a OpencorporaTagReg,
    pub para_id: Option<u16>
}

#[derive(Default, Debug, Clone)]
pub struct SeenSet{
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
