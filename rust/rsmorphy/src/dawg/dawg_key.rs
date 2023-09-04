use std::io;
use std::io::Cursor;
use std::io::Write;
use std::ops;
use std::str::from_utf8_unchecked;

// Longer keys are unlikely to be found in the dictionary, so we can use a fixed size buffer.
const BUF_SIZE: usize = 127;
const SEP: &str = ":";

#[derive(Debug, Clone, Copy)]
pub struct WordTagDawgKey {
    buf: [u8; BUF_SIZE],
    len: u8,
}

impl WordTagDawgKey {
    pub fn new(word_lower: &str, tag: &str) -> Option<Self> {
        let len = word_lower.len() + SEP.len() + tag.len();
        if len > BUF_SIZE {
            return None;
        }
        let mut buf = [0_u8; BUF_SIZE];
        let mut cur = Cursor::new(&mut buf[..]);
        let a = cur.write(word_lower.as_bytes());
        let b = cur.write(SEP.as_bytes());
        let c = cur.write(tag.as_bytes());

        let Ok(written) = [a, b, c].into_iter().sum::<io::Result<usize>>() else {
            return None;
        };
        debug_assert_eq!(len, written);

        let len = written as u8;
        Some(WordTagDawgKey { buf, len })
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // SAFETY: Cursor does not allow to write beyond the end of the buffer, and the result is
        // checked that no error has occurred.
        let slice = unsafe { self.buf.get_unchecked(..self.len as usize) };
        // SAFETY: Only valid UTF-8 has been written to the buffer from &str.
        unsafe { from_utf8_unchecked(slice) }
    }
}

impl ops::Deref for WordTagDawgKey {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for WordTagDawgKey {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
