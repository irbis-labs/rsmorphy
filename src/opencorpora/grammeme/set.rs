use std::collections::HashSet;

use super::Grammeme;


#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GrammemeSet {
    pub set: HashSet<Grammeme>
}


regex!(SEP, r"(,| )");


impl GrammemeSet {
    pub fn from_str<'s, S>(s: S) -> Self where S: AsRef<str> {
        GrammemeSet {
            set: SEP.split(s.as_ref())
                .map(|g: &str| Grammeme::new(g) )
                .collect()
        }
    }
}
