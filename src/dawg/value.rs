use util::u16_from_slice;

pub trait DawgValue {
    fn from_bytes(bytes: &[u8]) -> Self;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct HH(pub u16, pub u16);

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct HHH(pub u16, pub u16, pub u16);

impl DawgValue for HH {
    fn from_bytes(slice: &[u8]) -> Self {
        HH(
            u16::from_be(u16_from_slice(&slice[..2])),
            u16::from_be(u16_from_slice(&slice[2..])),
        )
    }
}

impl DawgValue for HHH {
    fn from_bytes(slice: &[u8]) -> Self {
        HHH(
            u16::from_be(u16_from_slice(&slice[..2])),
            u16::from_be(u16_from_slice(&slice[2..4])),
            u16::from_be(u16_from_slice(&slice[4..])),
        )
    }
}
