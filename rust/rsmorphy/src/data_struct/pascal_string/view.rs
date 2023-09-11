use std::borrow::Borrow;
use std::cmp;
use std::fmt;
use std::ops;
use std::str::from_utf8_unchecked;

#[derive(Debug, Clone, Copy)]
pub struct PascalStringView<'a> {
    pub(super) data: &'a [u8],
    pub(super) len: &'a u8,
}

impl<'a> PascalStringView<'a> {
    #[inline(always)]
    pub fn len(&self) -> usize {
        *self.len as usize
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &'a [u8] {
        unsafe { self.data.get_unchecked(..*self.len as usize) }
    }

    #[inline(always)]
    pub fn as_str(&self) -> &'a str {
        unsafe { from_utf8_unchecked(self.as_bytes()) }
    }
}

impl<'a> ops::Deref for PascalStringView<'a> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> Borrow<str> for PascalStringView<'a> {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self
    }
}

impl<'a> AsRef<[u8]> for PascalStringView<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> AsRef<str> for PascalStringView<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<'a> fmt::Display for PascalStringView<'a> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a, T: ops::Deref<Target = str>> PartialEq<T> for PascalStringView<'a> {
    #[inline(always)]
    fn eq(&self, other: &T) -> bool {
        self.as_str().eq(other.deref())
    }
}

impl<'a> Eq for PascalStringView<'a> {}

impl<'a, T: ops::Deref<Target = str>> PartialOrd<T> for PascalStringView<'a> {
    #[inline(always)]
    fn partial_cmp(&self, other: &T) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other.deref())
    }
}

impl<'a> Ord for PascalStringView<'a> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}
