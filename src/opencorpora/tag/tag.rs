use std::collections::BTreeSet;

use opencorpora::grammeme::Grammeme;


pub struct OTag {
    pub grammemes:      BTreeSet<Grammeme>,
}
