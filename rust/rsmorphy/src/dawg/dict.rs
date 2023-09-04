use std::ops;

use crate::dawg::dict_entry::DictEntry;
use crate::dawg::dict_entry::DictIndex;
use crate::dawg::dict_entry::DictLabel;
use crate::dawg::dict_entry::DictValue;

/// Dictionary class for retrieval and binary I/O.
#[derive(Debug, Clone)]
pub struct DawgDictionary {
    units: Box<[DictEntry]>,
}

impl DawgDictionary {
    pub const ROOT_INDEX: DictIndex = DictIndex::root();

    pub fn new(units: Box<[DictEntry]>) -> Self {
        DawgDictionary { units }
    }

    /// Gets a value from a given index if a given index is related to the end of a key.
    pub(in crate::dawg) fn try_value(&self, index: DictIndex) -> Option<DictValue> {
        let unit = self[index];
        if unit.has_leaf() {
            let value_index = index.next_index(unit.offset(), 0.into());
            Some(self[value_index].value())
        } else {
            None
        }
    }

    /// Exact matching.
    #[allow(dead_code)]
    pub fn contains(&self, key: &str) -> bool {
        self.follow_bytes(key, Self::ROOT_INDEX)
            .map(|index| index.as_entry().has_leaf())
            .unwrap_or(false)
    }

    /// Exact matching (returns value)
    pub fn find(&self, key: &str) -> Option<DictValue> {
        self.follow_bytes(key, Self::ROOT_INDEX)
            .and_then(|index| self.try_value(index))
    }

    /// Returns a list with keys of this DAWG that are prefixes of the `key`.
    fn _prefixes<'k>(&self, key: &'k &str, result: &mut Vec<&'k str>) {
        let mut index = Self::ROOT_INDEX;
        for (len, ch) in (1..).zip(key.as_bytes().iter().copied()) {
            let Some(new_index) = self.follow_byte(ch.into(), index) else {
                break;
            };
            index = new_index;
            if self[index].has_leaf() {
                result.push(&key[..len])
            };
        }
    }

    /// Follows a transition
    pub(in crate::dawg) fn follow_byte(
        &self,
        label: DictLabel,
        index: DictIndex,
    ) -> Option<DictIndex> {
        #[cfg(feature = "trace_dawg")]
        log::trace!(
            r#"Dictionary::follow_byte() label: {:x}, index = {:x} "#,
            label,
            index
        );

        let unit = self[index];

        #[cfg(feature = "trace_dawg")]
        log::trace!(r#"Dictionary::follow_byte() unit: {:x} "#, unit);

        let offset = unit.offset();

        #[cfg(feature = "trace_dawg")]
        log::trace!(r#"Dictionary::follow_byte() offset: {:x} "#, offset);

        // let next_index = (index ^ offset ^ label) & dict_entry::PRECISION_MASK;
        let next_index = index.next_index(offset, label);
        let leaf_label = self[next_index].label();

        #[cfg(feature = "trace_dawg")]
        log::trace!(r#"Dictionary::follow_byte() leaf_label: {:x} "#, leaf_label);

        if leaf_label == label {
            Some(next_index)
        } else {
            None
        }
    }

    /// Follows transitions.
    pub(in crate::dawg) fn follow_bytes(
        &self,
        key: &str,
        mut index: DictIndex,
    ) -> Option<DictIndex> {
        for &ch in key.as_bytes() {
            index = self.follow_byte(ch.into(), index)?;
        }
        Some(index)
    }
}

impl ops::Index<DictIndex> for DawgDictionary {
    type Output = DictEntry;

    #[inline(always)]
    fn index(&self, index: DictIndex) -> &Self::Output {
        assert!(index.as_usize() < self.units.len(), "index out of bounds");
        unsafe { self.units.get_unchecked(index.as_usize()) }
    }
}
