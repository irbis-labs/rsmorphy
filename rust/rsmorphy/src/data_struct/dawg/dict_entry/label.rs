use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DictLabel(pub(super) u32);

impl DictLabel {
    #[inline(always)]
    pub const fn from_u8(byte: u8) -> Self {
        DictLabel(byte as u32)
    }
}

impl From<u8> for DictLabel {
    #[inline(always)]
    fn from(byte: u8) -> Self {
        DictLabel::from_u8(byte)
    }
}

impl fmt::LowerHex for DictLabel {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
