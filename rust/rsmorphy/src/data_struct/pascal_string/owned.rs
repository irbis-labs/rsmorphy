use std::borrow::Borrow;
use std::cmp;
use std::fmt;
use std::ops;

use super::view::PascalStringView;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PascalString<const MAX_LEN: usize> {
    len: u8,
    data: [u8; MAX_LEN],
}

impl<const MAX_LEN: usize> PascalString<MAX_LEN> {
    pub const MAX_LEN: usize = {
        assert!(
            MAX_LEN <= u8::MAX as usize,
            "PascalString max length is 255"
        );
        MAX_LEN
    };

    #[inline(always)]
    pub fn new() -> Self {
        // This leads to a compile time error, if MAX_LEN > 255:
        assert!(Self::MAX_LEN <= u8::MAX as usize);

        Self {
            len: 0,
            data: [0; MAX_LEN],
        }
    }

    /// Creates a new `PascalString` from a `&str`.
    /// If the length of the string exceeds `MAX_LEN`,
    /// the string is truncated at the nearest valid UTF-8 boundary
    /// to ensure its length does not exceed `MAX_LEN`.
    #[inline(always)]
    pub fn from_str_truncated(string: impl AsRef<str>) -> Self {
        Self::from_str_truncated_(string.as_ref())
    }

    #[inline]
    fn from_str_truncated_(string: &str) -> Self {
        use std::fmt::Write;

        let mut ps = Self::new();
        for s in string.split("").skip(1) {
            if ps.write_str(s).is_err() {
                break;
            };
        }
        ps
    }

    #[inline(always)]
    pub fn into_inner(self) -> (u8, [u8; MAX_LEN]) {
        (self.len, self.data)
    }

    #[inline(always)]
    fn as_view(&self) -> PascalStringView<'_> {
        PascalStringView {
            data: &self.data,
            len: &self.len,
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_view().as_bytes()
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.as_view().as_str()
    }
}

impl<const MAX_LEN: usize> Default for PascalString<MAX_LEN> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_LEN: usize> fmt::Display for PascalString<MAX_LEN> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<const MAX_LEN: usize> ops::Deref for PascalString<MAX_LEN> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const MAX_LEN: usize> Borrow<str> for PascalString<MAX_LEN> {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self
    }
}

impl<const MAX_LEN: usize> AsRef<str> for PascalString<MAX_LEN> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self
    }
}

impl<const MAX_LEN: usize> AsRef<[u8]> for PascalString<MAX_LEN> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<T: ops::Deref<Target = str>, const MAX_LEN: usize> PartialEq<T> for PascalString<MAX_LEN> {
    #[inline(always)]
    fn eq(&self, other: &T) -> bool {
        self.as_str().eq(other.deref())
    }
}

impl<const MAX_LEN: usize> Eq for PascalString<MAX_LEN> {}

impl<T: ops::Deref<Target = str>, const MAX_LEN: usize> PartialOrd<T> for PascalString<MAX_LEN> {
    #[inline(always)]
    fn partial_cmp(&self, other: &T) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other.deref())
    }
}

impl<const MAX_LEN: usize> Ord for PascalString<MAX_LEN> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<'a, const MAX_LEN: usize> TryFrom<&'a str> for PascalString<MAX_LEN> {
    type Error = ();

    #[inline]
    fn try_from(value: &'a str) -> Result<PascalString<MAX_LEN>, Self::Error> {
        let bytes = value.as_bytes();
        let len = bytes.len();

        if len > MAX_LEN {
            return Err(());
        }

        let data = match <[u8; MAX_LEN]>::try_from(bytes).ok() {
            Some(data) => data,
            None => {
                let mut data = [0; MAX_LEN];
                data[..len].copy_from_slice(bytes);
                data
            }
        };

        let len = len as u8;

        Ok(PascalString { len, data })
    }
}

impl<const MAX_LEN: usize> TryFrom<char> for PascalString<MAX_LEN> {
    type Error = ();

    #[inline]
    fn try_from(value: char) -> Result<PascalString<MAX_LEN>, Self::Error> {
        PascalString::try_from(value.encode_utf8(&mut [0; 4]).as_ref())
    }
}

impl<const MAX_LEN: usize> fmt::Write for PascalString<MAX_LEN> {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let begin = self.len();
        let end = begin + s.len();

        if end > MAX_LEN {
            return Err(fmt::Error);
        }

        self.data[begin..end].copy_from_slice(s.as_bytes());
        self.len = end as u8;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn test_eq() {
        use std::fmt::Write;

        let s = String::from("abc");
        let ps = PascalString::<4>::try_from("abc").unwrap();

        assert_eq!(ps, s);
        assert_eq!(ps.as_view(), s);
        assert_eq!(ps.as_view(), ps);

        let s = String::from("abcd");
        let mut ps = PascalString::<4>::new();
        write!(&mut ps, "abcd").unwrap();

        assert_eq!(ps, s);
    }

    #[test]
    fn test_ord() {
        let ps1 = PascalString::<4>::try_from("abc").unwrap();
        let ps2 = PascalString::<4>::try_from("abcd").unwrap();

        assert!(ps1 < ps2);
        assert!(ps1 <= ps2);
        assert!(ps2 > ps1);
        assert!(ps2 >= ps1);
    }

    #[test]
    fn test_size() {
        assert_eq!(mem::size_of::<PascalString<0>>(), 1);
        assert_eq!(mem::size_of::<PascalString<1>>(), 2);
        assert_eq!(mem::size_of::<PascalString<2>>(), 3);
        assert_eq!(mem::size_of::<PascalString<3>>(), 4);
        assert_eq!(mem::size_of::<PascalString<4>>(), 5);
    }

    // Should not compile with a compile error:
    // "the evaluated program panicked at 'PascalString max length is 255'"
    // #[test]
    // fn test_max_size() {
    //     let _x = PascalString::<256>::new();
    // }
}
