use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;
use std::path::Path;

use base64;
use flate2::read::GzDecoder;

use dawg::completer::Completer;
use dawg::dictionary::Dictionary;
use dawg::guide::Guide;
use dawg::value::DawgValue;

const PAYLOAD_SEPARATOR: &str = "\x01";

#[derive(Debug, Clone)]
pub struct Dawg {
    dict: Dictionary,
}

impl Dawg {
    pub fn from_file<P>(p: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::from_reader(&mut GzDecoder::new(File::open(p).unwrap()))
    }

    pub fn from_reader<T>(fp: &mut T) -> Self
    where
        T: Read,
    {
        Dawg {
            dict: Dictionary::from_reader(fp),
        }
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
                result.push(&key[..i + 1])
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

#[derive(Debug, Clone)]
pub struct CompletionDawg<V>
where
    V: DawgValue,
{
    dawg: Dawg,
    guide: Guide,
    _phantom: PhantomData<V>,
}

impl<V> CompletionDawg<V>
where
    V: DawgValue,
{
    pub fn from_file<P>(p: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::from_reader(&mut GzDecoder::new(File::open(p).unwrap()))
    }

    pub fn from_reader<T>(fp: &mut T) -> Self
    where
        T: Read,
    {
        CompletionDawg {
            dawg: Dawg::from_reader(fp),
            guide: Guide::from_reader(fp),
            _phantom: PhantomData,
        }
    }

    /// Returns a list of (key, value) tuples for all variants of `key`
    /// in this DAWG according to `replaces`.
    ///
    /// `replaces` is an object obtained from
    /// `DAWG.compile_replaces(mapping)` where mapping is a dict
    /// that maps single-char unicode sitrings to another single-char
    /// unicode strings.
    pub fn similar_items(
        &self,
        key: &str,
        replaces: &BTreeMap<String, String>,
    ) -> Vec<(String, Vec<V>)> {
        let mut result: Vec<(String, Vec<V>)> = Vec::new();
        self.similar_items_(&mut result, "", key, self.dawg.dict.root, replaces);
        result
    }

    fn similar_items_(
        &self,
        result: &mut Vec<(String, Vec<V>)>,
        current_prefix: &str,
        key: &str,
        mut index: u32,
        replace_chars: &BTreeMap<String, String>,
    ) {
        trace!(r#"DAWG::similar_items_() index: {}"#, index);

        let start_pos = current_prefix.len();
        let subkey = &key[start_pos..];

        let mut word_pos = start_pos;

        for b_step in subkey.split("").filter(|v| !v.is_empty()) {
            trace!(r#" b_step: {:?}"#, b_step);

            if let Some(replace_char) = replace_chars.get(b_step) {
                trace!(
                    r#" b_step in replace_chars ({:?} => {:?})"#,
                    b_step,
                    replace_char
                );

                if let Some(next_index) = self.dawg.dict.follow_bytes(replace_char, index) {
                    trace!(r#" next_index: {}"#, next_index);
                    let prefix = format!(
                        "{}{}{}",
                        current_prefix,
                        &key[start_pos..word_pos],
                        replace_char
                    );
                    self.similar_items_(result, &prefix, key, next_index, replace_chars);
                };
            }
            index = match self.dawg.dict.follow_bytes(b_step, index) {
                Some(v) => v,
                None => return,
            };
            trace!(r#" index: {}"#, index);
            word_pos += b_step.len()
        }
        if let Some(index) = self.dawg.dict.follow_bytes(PAYLOAD_SEPARATOR, index) {
            trace!(r#" index: {}"#, index);
            let found_key = format!("{}{}", current_prefix, subkey);
            trace!(r#" found_key: {}"#, found_key);
            let value = self.value_for_index_(index);
            result.insert(0, (found_key, value));
        }
    }

    fn value_for_index_(&self, index: u32) -> Vec<V> {
        trace!(r#"DAWG::value_for_index_(index: {}) "#, index);
        let mut result: Vec<V> = Vec::new();
        let mut completer = Completer::new(&self.dawg.dict, &self.guide, index, "");
        while let Some(key) = completer.next_key() {
            trace!(r#"DAWG::value_for_index_(...); key: "{:?}" "#, key);
            let value = V::new_in_place(move |buf| {
                let decoded = base64::decode_config_slice(&key, base64::STANDARD, buf).unwrap();
                trace!(r#"DAWG::value_for_index_(...); bytes: {:?} "#, buf);
                assert_eq!(decoded, buf.len());
            });
            result.push(value);
        }
        result
    }

    pub fn prefixes<'k>(&self, key: &'k str) -> Vec<&'k str> {
        self.dawg.prefixes(key)
    }

    pub fn find(&self, key: &str) -> Option<u32> {
        self.dawg.dict.find(key)
    }
}
