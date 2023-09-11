use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DictValue(pub(super) u32);

impl DictValue {
    #[inline(always)]
    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::LowerHex for DictValue {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
