use std::io;
use std::mem;

use super::super::DawgValue;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct HH(pub u16, pub u16);

impl DawgValue for HH {
    #[inline(always)]
    fn new_in_place<F>(f: F) -> io::Result<Self>
    where
        F: FnOnce(&mut [u8]) -> io::Result<()>,
        Self: Sized,
    {
        let mut buf = [0_u8; 4];
        f(&mut buf)?;
        let HH(a, b) = unsafe { mem::transmute(buf) };
        Ok(HH(h(a), h(b)))
    }
}

fn h(v: u16) -> u16 {
    u16::from_be(v)
}
