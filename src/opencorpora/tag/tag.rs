use std::collections::BTreeSet;

use crate::opencorpora::grammeme::Grammeme;

#[derive(Debug)]
pub struct OTag {
    pub grammemes: BTreeSet<Grammeme>,
}
