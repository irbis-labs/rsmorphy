use std::borrow::Borrow;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops;

use smart_string::PascalString;

use super::GramIdError;

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct GramId {
    inner: PascalString<{ Self::MAX_LEN }>,
}

impl GramId {
    pub const MAX_LEN: usize = 11;

    #[inline(always)]
    pub fn from_str(gram_lat: &str) -> Result<Self, GramIdError> {
        let bytes = gram_lat.as_bytes();

        let len = bytes.len();
        if len < 1 || len > Self::MAX_LEN {
            return Err(GramIdError::InvalidLength(len));
        }

        // PANIC SAFETY: Only the length can be invalid, and it is checked above.
        let inner = PascalString::try_from(gram_lat).unwrap();

        if !bytes.iter().all(u8::is_ascii) {
            let (_len, data) = inner.into_inner();
            return Err(GramIdError::NotAscii(data));
        }

        Ok(Self { inner })
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // SAFETY: GramId is always constructed from &str.
        self.inner.as_str()
    }
}

impl fmt::Display for GramId {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl fmt::Debug for GramId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("GramId").field(&self.as_str()).finish()
    }
}

impl Hash for GramId {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl AsRef<str> for GramId {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self
    }
}

impl ops::Deref for GramId {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for GramId {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self
    }
}
