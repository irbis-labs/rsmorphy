use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct GuideEntry {
    pub child: u8,
    pub sibling: u8,
}

impl GuideEntry {
    pub fn new(child: u8, sibling: u8) -> Self {
        GuideEntry { child, sibling }
    }
}
