use std::fmt::Write;
use std::io;
use std::marker::PhantomData;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use smallvec::SmallVec;
use smart_string::DisplayExt;
use smart_string::SmartString;

use super::completer::Completer;
use super::dict::DawgDictionarySuspended;
use super::dict_entry::DictIndex;
use super::dict_entry::DictValue;
use super::guide::DawgGuide;
use super::guide::DawgGuideSuspended;
use super::value::DawgValue;
use super::DawgDictionary;
use crate::collections::letter_pairs::LetterPairs;
use crate::cryostasis::Revive;
use crate::cryostasis::Suspend;

const PAYLOAD_SEPARATOR: &str = "\x01";

#[derive(Debug)]
pub struct CompletionDawg<'d, V>
where
    V: DawgValue,
{
    dict: DawgDictionary<'d>,
    guide: DawgGuide<'d>,
    _value: PhantomData<V>,
}

#[derive(Debug)]
pub struct CompletionDawgSuspended<V>
where
    V: DawgValue,
{
    dict: DawgDictionarySuspended,
    guide: DawgGuideSuspended,
    _value: PhantomData<V>,
}

impl<V> Revive for CompletionDawgSuspended<V>
where
    V: DawgValue,
{
    type Revived<'d> = CompletionDawg<'d, V>;

    #[inline(always)]
    unsafe fn revive<'d>(&self, source_bin: &'d [u8]) -> CompletionDawg<'d, V> {
        CompletionDawg {
            dict: self.dict.revive(source_bin),
            guide: self.guide.revive(source_bin),
            _value: PhantomData,
        }
    }
}

impl<'d, V> Suspend for CompletionDawg<'d, V>
where
    V: DawgValue,
{
    type Suspended = CompletionDawgSuspended<V>;

    #[inline(always)]
    fn suspend(&self) -> Self::Suspended {
        CompletionDawgSuspended {
            dict: self.dict.suspend(),
            guide: self.guide.suspend(),
            _value: PhantomData,
        }
    }
}

impl<'d, V> Clone for CompletionDawg<'d, V>
where
    V: DawgValue,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        CompletionDawg {
            dict: self.dict,
            guide: self.guide,
            _value: PhantomData,
        }
    }
}

impl<'d, V> Copy for CompletionDawg<'d, V> where V: DawgValue {}

impl<'d, V> CompletionDawg<'d, V>
where
    V: DawgValue,
{
    pub fn new(dict: DawgDictionary<'d>, guide: DawgGuide<'d>) -> Self {
        // TODO validate dict and guide
        CompletionDawg {
            dict,
            guide,
            _value: PhantomData,
        }
    }

    pub fn into_inner(self) -> (DawgDictionary<'d>, DawgGuide<'d>) {
        (self.dict, self.guide)
    }

    #[inline(always)]
    pub fn find(&self, key: &str) -> Option<DictValue> {
        self.dict.find(key)
    }

    /// Returns a list of (key, value) tuples for all variants of `key`
    /// in this DAWG according to `replaces`.
    ///
    /// `replaces` is an object obtained from
    /// `DAWG.compile_replaces(mapping)` where mapping is a dict
    /// that maps single-char unicode strings to another single-char
    /// unicode strings.
    pub fn similar_items(
        &self,
        key: &str,
        replaces: LetterPairs<'_>,
    ) -> io::Result<SmallVec<[(SmartString, SmallVec<[V; 8]>); 4]>> {
        let mut result = SmallVec::new();
        self.similar_items_(&mut result, "", key, DawgDictionary::ROOT_INDEX, replaces)?;
        Ok(result)
    }

    fn similar_items_(
        &self,
        result: &mut SmallVec<[(SmartString, SmallVec<[V; 8]>); 4]>,
        current_prefix: &str,
        key: &str,
        mut index: DictIndex,
        replace_chars: LetterPairs<'_>,
    ) -> io::Result<()> {
        #[cfg(feature = "log_trace_dawg")]
        log::trace!(r#"DAWG::similar_items_() index: {:x}"#, index);

        let start_pos = current_prefix.len();
        let subkey = &key[start_pos..];

        let mut word_pos = start_pos;

        for b_step in subkey.split("").filter(|v| !v.is_empty()) {
            #[cfg(feature = "log_trace_dawg")]
            log::trace!(r#" b_step: {:?}"#, b_step);

            if let Some(replace_char) = replace_chars.find_pair(b_step) {
                #[cfg(feature = "log_trace_dawg")]
                log::trace!(
                    r#" b_step in replace_chars ({:?} => {:?})"#,
                    b_step,
                    replace_char
                );

                if let Some(next_index) = self
                    .dict
                    .follow_bytes(replace_char.encode_utf8(&mut [0u8; 4]).as_ref(), index)
                {
                    #[cfg(feature = "log_trace_dawg")]
                    log::trace!(r#" next_index: {:x}"#, next_index);

                    let mut prefix: SmartString =
                        format_args!("{current_prefix}{}", &key[start_pos..word_pos]).to_fmt();
                    prefix.write_char(replace_char).unwrap();
                    self.similar_items_(result, &prefix, key, next_index, replace_chars)?;
                };
            }
            index = match self.dict.follow_bytes(b_step, index) {
                Some(v) => v,
                None => return Ok(()),
            };

            #[cfg(feature = "log_trace_dawg")]
            log::trace!(r#" index: {:x}"#, index);

            word_pos += b_step.len()
        }

        if let Some(index) = self.dict.follow_bytes(PAYLOAD_SEPARATOR, index) {
            #[cfg(feature = "log_trace_dawg")]
            log::trace!(r#" index: {:x}"#, index);

            let found_key = format_args!("{current_prefix}{subkey}").to_fmt();

            #[cfg(feature = "log_trace_dawg")]
            log::trace!(r#" found_key: {}"#, found_key);

            let list = self.values_for_index(index)?;
            result.push((found_key, list));
        }

        Ok(())
    }

    fn values_for_index(&self, index: DictIndex) -> io::Result<SmallVec<[V; 8]>> {
        #[cfg(feature = "log_trace_dawg")]
        log::trace!(r#"DAWG::value_for_index_(index: {:x}) "#, index);

        let mut result = SmallVec::new();
        let mut completer = Completer::new(self.dict, self.guide, index, "");
        while let Some(key) = completer.next_key() {
            #[cfg(feature = "log_trace_dawg")]
            log::trace!(r#"DAWG::value_for_index_(...); key: "{:?}" "#, key);

            let value = V::new_in_place(move |buf| {
                let decoded = STANDARD
                    .decode_slice_unchecked(&key, buf)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                #[cfg(feature = "log_trace_dawg")]
                log::trace!(r#"DAWG::value_for_index_(...); bytes: {:?} "#, buf);

                debug_assert_eq!(decoded, buf.len());
                Ok(())
            });
            result.push(value?);
        }
        Ok(result)
    }
}
