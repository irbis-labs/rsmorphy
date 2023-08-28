use std::io;
use std::mem;

use crate::dawg::DawgValue;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct HHH(pub u16, pub u16, pub u16);

impl DawgValue for HHH {
    #[inline(always)]
    fn new_in_place<F>(f: F) -> io::Result<Self>
    where
        F: FnOnce(&mut [u8]) -> io::Result<()>,
        Self: Sized,
    {
        let mut buf = [0_u8; 6];
        f(&mut buf)?;
        let HHH(a, b, c) = unsafe { mem::transmute(buf) };
        Ok(HHH(h(a), h(b), h(c)))
    }
}

fn h(v: u16) -> u16 {
    u16::from_be(v)
}
