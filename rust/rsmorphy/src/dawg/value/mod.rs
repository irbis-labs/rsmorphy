use std::io;

mod hh;
mod hhh;

pub use self::hh::HH;
pub use self::hhh::HHH;

pub trait DawgValue {
    fn new_in_place<F>(f: F) -> io::Result<Self>
    where
        F: FnOnce(&mut [u8]) -> io::Result<()>,
        Self: Sized;
}
