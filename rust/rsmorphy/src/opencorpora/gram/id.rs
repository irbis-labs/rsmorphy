use std::borrow::Borrow;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops;

use serde::Deserialize;
use serde::Serialize;

use super::GramIdError;

#[derive(Serialize, Deserialize)]
//
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct GramId {
    // ASCII bytes, e.g. b"NOUN".
    id: [u8; 4],
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
        if bytes.len() != 4 {
            return Err(GramIdError::InvalidLength(bytes.len()));
        }
        let mut id = [0; 4];
        id.copy_from_slice(bytes);
        if !id.iter().all(u8::is_ascii) {
            return Err(GramIdError::NotAscii(id));
        }
        Ok(Self { id })
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // SAFETY: GramId is always constructed from &str.
        unsafe { std::str::from_utf8_unchecked(&self.id) }
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
