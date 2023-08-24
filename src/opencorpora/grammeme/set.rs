use std::collections::HashSet;
use std::ops;

use crate::opencorpora::grammeme::Grammeme;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GrammemeSet {
    pub set: HashSet<Grammeme>,
}

regex!(SEP, r"(,| )");

impl GrammemeSet {
    pub fn parse_fmt_int<S>(s: S) -> Self
    where
        S: AsRef<str>,
    {
        GrammemeSet {
            set: SEP.split(s.as_ref()).map(Grammeme::new).collect(),
        }
    }
}

impl ops::Deref for GrammemeSet {
    type Target = HashSet<Grammeme>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

impl ops::DerefMut for GrammemeSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.set
    }
}
