use std::borrow::Borrow;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops;

use super::GramIdError;

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct GramId {
    // ASCII bytes, e.g. b"NOUN".
    id: [u8; Self::MAX_LEN],
    len: u8,
}

impl GramId {
    pub const MAX_LEN: usize = 11;
}

impl fmt::Debug for GramId {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Hash for GramId {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl GramId {
    #[inline(always)]
    pub fn from_str(gram_lat: &str) -> Result<Self, GramIdError> {
        let bytes = gram_lat.as_bytes();

        let len = bytes.len();
        if len < 1 || len > Self::MAX_LEN {
            return Err(GramIdError::InvalidLength(len));
        }

        let mut id = [0; Self::MAX_LEN];
        id[..len].copy_from_slice(bytes);
        if !id[..len].iter().all(u8::is_ascii) {
            return Err(GramIdError::NotAscii(id));
        }

        let len = len as u8;
        Ok(Self { id, len })
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // SAFETY: GramId is always constructed from &str.
        unsafe { std::str::from_utf8_unchecked(self.id.get_unchecked(..self.len as usize)) }
    }
}

impl AsRef<str> for GramId {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
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
        self.as_str()
    }
}

impl fmt::Display for GramId {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
