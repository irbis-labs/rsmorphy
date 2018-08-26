use std::mem;

pub trait DawgValue {
    #[inline(always)]
    fn new_in_place<F>(f: F) -> Self where F: FnOnce(&mut [u8]);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct HH(pub u16, pub u16);

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct HHH(pub u16, pub u16, pub u16);

impl DawgValue for HH {
    #[inline(always)]
    fn new_in_place<F>(f: F) -> Self
    where
        F: FnOnce(&mut [u8]),
    {
        let mut buf = [0_u8; 4];
        f(&mut buf);
        let buf: HH = unsafe { mem::transmute(buf) };
        HH(
            u16::from_be(buf.0),
            u16::from_be(buf.1)
        )
    }
}

impl DawgValue for HHH {
    #[inline(always)]
    fn new_in_place<F>(f: F) -> Self
        where
            F: FnOnce(&mut [u8]),
    {
        let mut buf = [0_u8; 6];
        f(&mut buf);
        let buf: HHH = unsafe { mem::transmute(buf) };
        HHH(
            u16::from_be(buf.0),
            u16::from_be(buf.1),
            u16::from_be(buf.2)
        )
    }
}
