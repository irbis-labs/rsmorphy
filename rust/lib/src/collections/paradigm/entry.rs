#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C, align(2))]
pub struct ParadigmEntry {
    pub(crate) prefix_id: u16,
    pub(crate) tag_id: u16,
    pub(crate) suffix_id: u16,
}

impl ParadigmEntry {
    #[inline(always)]
    pub(crate) fn new(prefix_id: u16, tag_id: u16, suffix_id: u16) -> Self {
        ParadigmEntry {
            prefix_id,
            tag_id,
            suffix_id,
        }
    }

    #[inline(always)]
    pub fn prefix_id(&self) -> u16 {
        self.prefix_id
    }

    #[inline(always)]
    pub fn tag_id(&self) -> u16 {
        self.tag_id
    }

    #[inline(always)]
    pub fn suffix_id(&self) -> u16 {
        self.suffix_id
    }
}
