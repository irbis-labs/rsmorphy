use std::collections::HashSet;

use crate::opencorpora::grammeme::Grammeme;

#[derive(Clone, Debug, Default)]
pub struct GrammemeMeta {
    // XXX remove
    pub index: usize,
    pub children: HashSet<Grammeme>,
    pub incompatible: HashSet<Grammeme>,
}
