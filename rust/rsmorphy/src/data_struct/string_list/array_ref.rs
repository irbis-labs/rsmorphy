use std::str::from_utf8_unchecked;

use crate::data_struct::string_list::StringArrayIter;
use crate::data_struct::string_list::StringArraySuspended;
use crate::model::dictionary::data_format::binpack::Suspend;

#[derive(Debug, Clone, Copy)]
pub struct StringArrayRef<'a> {
    pub(super) data: &'a [u8],
    pub(super) ends: &'a [u32],
}

impl<'a> StringArrayRef<'a> {
    pub unsafe fn new_unchecked(data: &'a [u8], ends: &'a [u32]) -> Self {
        StringArrayRef { data, ends }
    }

    pub fn into_inner(self) -> (&'a [u8], &'a [u32]) {
        (self.data, self.ends)
    }

    pub fn len(self) -> usize {
        self.ends.len()
    }

    pub fn is_empty(self) -> bool {
        self.ends.is_empty()
    }

    pub fn get(self, index: usize) -> Option<&'a str> {
        if index >= self.ends.len() {
            return None;
        }
        // SAFETY: index is checked above.
        Some(unsafe { self.get_unchecked(index) })
    }

    pub unsafe fn get_unchecked(self, index: usize) -> &'a str {
        let start = if index > 0 {
            // SAFETY: index is checked above.
            let index = index - 1;
            // SAFETY: it is on the caller to ensure that the index is in bounds.
            unsafe { *self.ends.get_unchecked(index) as usize }
        } else {
            0
        };
        // SAFETY: it is on the caller to ensure that the index is in bounds.
        let end = unsafe { *self.ends.get_unchecked(index) as usize };
        // SAFETY: self.ends is constructed in a way that ensures that indexes are in bounds.
        let slice = unsafe { self.data.get_unchecked(start..end) };
        // SAFETY: self.data and self.ends are constructed in a way that ensures that the data
        //   is valid UTF-8.
        unsafe { from_utf8_unchecked(slice) }
    }

    pub fn iter(self) -> StringArrayIter<'a> {
        StringArrayIter::new(self)
    }
}

impl Suspend for StringArrayRef<'_> {
    type Suspended = StringArraySuspended;

    fn suspend(&self) -> Self::Suspended {
        StringArraySuspended {
            data: self.data as *const _,
            ends: self.ends as *const _,
        }
    }
}
