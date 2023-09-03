use std::collections::HashSet;
use std::ops;

use super::Seen;

// TODO Replace with a SmallVec-like alternative for HashSet.
#[derive(Default, Debug, Clone)]
pub struct SeenSet<'a> {
    set: HashSet<Seen<'a>>,
}

impl<'a> SeenSet<'a> {
    pub fn new() -> Self {
        let set = HashSet::new();
        SeenSet { set }
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
