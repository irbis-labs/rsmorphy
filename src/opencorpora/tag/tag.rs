use std::collections::BTreeSet;

use opencorpora::grammeme::Grammeme;

#[derive(Debug)]
pub struct OTag {
    pub grammemes: BTreeSet<Grammeme>,
}
