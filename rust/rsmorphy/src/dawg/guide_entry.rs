#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GuideEntry {
    pub child: u8,
    pub sibling: u8,
}

impl GuideEntry {
    #[inline(always)]
    pub const fn new(child: u8, sibling: u8) -> Self {
        GuideEntry { child, sibling }
    }
}
