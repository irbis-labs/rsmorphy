use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(align(64))] // Align to cache lines
pub struct Align64<T>(pub T);

impl<T> ops::Deref for Align64<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
