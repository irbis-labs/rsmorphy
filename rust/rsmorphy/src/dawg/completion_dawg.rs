use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::marker::PhantomData;
use std::path::Path;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use flate2::read::GzDecoder;
use inlinable_string::InlinableString;
use inlinable_string::StringExt;
use smallvec::SmallVec;

use crate::dawg::completer::Completer;
use crate::dawg::guide::Guide;
use crate::dawg::value::DawgValue;
use crate::dawg::Dawg;
use crate::opencorpora::dictionary::LoadError;

const PAYLOAD_SEPARATOR: &str = "\x01";

#[derive(Debug)]
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
    pub fn from_file(p: impl AsRef<Path>) -> Result<Self, LoadError> {
        Self::from_reader(GzDecoder::new(File::open(p)?))
    }

    pub fn from_reader(mut reader: impl Read) -> Result<Self, LoadError> {
        Ok(CompletionDawg {
            dawg: Dawg::from_reader(&mut reader)?,
            guide: Guide::from_reader(&mut reader)?,
            _phantom: PhantomData,
        })
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
        replaces: &HashMap<InlinableString, InlinableString>,
    ) -> io::Result<SmallVec<[(InlinableString, SmallVec<[V; 8]>); 4]>> {
        let mut result = SmallVec::new();
        self.similar_items_(&mut result, "", key, self.dawg.dict.root, replaces)?;
        Ok(result)
    }

    fn similar_items_(
        &self,
        result: &mut SmallVec<[(InlinableString, SmallVec<[V; 8]>); 4]>,
        current_prefix: &str,
        key: &str,
        mut index: u32,
        replace_chars: &HashMap<InlinableString, InlinableString>,
    ) -> io::Result<()> {
        #[cfg(feature = "trace_dawg")]
        log::trace!(r#"DAWG::similar_items_() index: {}"#, index);

        let start_pos = current_prefix.len();
        let subkey = &key[start_pos..];

        let mut word_pos = start_pos;

        for b_step in subkey.split("").filter(|v| !v.is_empty()) {
            #[cfg(feature = "trace_dawg")]
            log::trace!(r#" b_step: {:?}"#, b_step);

            if let Some(replace_char) = replace_chars.get(b_step) {
                #[cfg(feature = "trace_dawg")]
                log::trace!(
                    r#" b_step in replace_chars ({:?} => {:?})"#,
                    b_step,
                    replace_char
                );

                if let Some(next_index) = self.dawg.dict.follow_bytes(replace_char, index) {
                    #[cfg(feature = "trace_dawg")]
                    log::trace!(r#" next_index: {}"#, next_index);
                    let mut prefix = InlinableString::new();
                    prefix.push_str(current_prefix);
                    prefix.push_str(&key[start_pos..word_pos]);
                    prefix.push_str(replace_char);
                    self.similar_items_(result, &prefix, key, next_index, replace_chars)?;
                };
            }
            index = match self.dawg.dict.follow_bytes(b_step, index) {
                Some(v) => v,
                None => return Ok(()),
            };
            #[cfg(feature = "trace_dawg")]
            log::trace!(r#" index: {}"#, index);
            word_pos += b_step.len()
        }

        if let Some(index) = self.dawg.dict.follow_bytes(PAYLOAD_SEPARATOR, index) {
            #[cfg(feature = "trace_dawg")]
            log::trace!(r#" index: {}"#, index);
            let found_key = InlinableString::from(current_prefix) + subkey;
            #[cfg(feature = "trace_dawg")]
            log::trace!(r#" found_key: {}"#, found_key);
            let value = self.value_for_index_(index)?;
            result.insert(0, (found_key, value));
        }

        Ok(())
    }

    fn value_for_index_(&self, index: u32) -> io::Result<SmallVec<[V; 8]>> {
        #[cfg(feature = "trace_dawg")]
        log::trace!(r#"DAWG::value_for_index_(index: {}) "#, index);
        let mut result = SmallVec::new();
        let mut completer = Completer::new(&self.dawg.dict, &self.guide, index, "");
        while let Some(key) = completer.next_key() {
            #[cfg(feature = "trace_dawg")]
            log::trace!(r#"DAWG::value_for_index_(...); key: "{:?}" "#, key);
            let value = V::new_in_place(move |buf| {
                let decoded = STANDARD
                    .decode_slice_unchecked(&key, buf)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                #[cfg(feature = "trace_dawg")]
                log::trace!(r#"DAWG::value_for_index_(...); bytes: {:?} "#, buf);
                debug_assert_eq!(decoded, buf.len());
                Ok(())
            });
            result.push(value?);
        }
        Ok(result)
    }

    pub fn prefixes<'k>(&self, key: &'k str) -> Vec<&'k str> {
        self.dawg.prefixes(key)
    }

    pub fn find(&self, key: &str) -> Option<u32> {
        self.dawg.dict.find(key)
    }
}
