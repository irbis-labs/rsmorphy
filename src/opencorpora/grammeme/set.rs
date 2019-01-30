use std::collections::HashSet;

use crate::opencorpora::grammeme::Grammeme;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GrammemeSet {
    pub set: HashSet<Grammeme>,
}

regex!(SEP, r"(,| )");

impl GrammemeSet {
    pub fn new<S>(s: S) -> Self
    where
        S: AsRef<str>,
    {
        GrammemeSet {
            set: SEP.split(s.as_ref()).map(Grammeme::new).collect(),
        }
    }
}
