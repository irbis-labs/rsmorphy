use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use flate2::read::GzDecoder;
use serde::Deserialize;
use serde::Serialize;

use crate::dawg::dictionary::Dictionary;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dawg {
    pub(super) dict: Dictionary,
}

impl Dawg {
    pub fn from_file(p: impl AsRef<Path>) -> io::Result<Self> {
        Self::from_reader(GzDecoder::new(File::open(p)?))
    }

    pub fn from_reader(reader: impl Read) -> io::Result<Self> {
        let dict = Dictionary::from_reader(reader)?;
        Ok(Dawg { dict })
    }

    /// Returns a list with keys of this DAWG that are prefixes of the `key`.
    pub fn prefixes<'k>(&self, key: &'k str) -> Vec<&'k str> {
        let mut result = Vec::new();
        let mut index = self.dict.root;
        for (i, &ch) in key.as_bytes().iter().enumerate() {
            index = match self.dict.follow_char(ch, index) {
                Some(v) => v,
                None => break,
            };
            if self.dict.has_value(index) {
                result.push(&key[..=i])
            };
        }
        result
    }

    pub fn sorted_prefixes<'k>(&self, key: &'k str) -> Vec<&'k str> {
        let mut result = self.prefixes(key);
        result.sort_by_key(|v| -(v.len() as isize));
        result
    }
}
