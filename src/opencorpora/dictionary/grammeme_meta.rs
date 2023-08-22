use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

use crate::opencorpora::grammeme::Grammeme;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct GrammemeMeta {
    // XXX remove
    pub index: usize,
    pub children: HashSet<Grammeme>,
    pub incompatible: HashSet<Grammeme>,
}
