use crate::collections::str_stack::list::StrList;
use crate::collections::str_stack::ref_::StrListRef;
use crate::collections::str_stack::StrListIter;

#[derive(Debug, Default)]
pub struct StrStack {
    pub(super) data: Vec<u8>,
    pub(super) ends: Vec<u32>,
}

impl StrStack {
    pub fn new() -> Self {
        StrStack::default()
    }

    pub fn build(self) -> StrList {
        StrList {
            data: self.data.into_boxed_slice(),
            ends: self.ends.into_boxed_slice(),
        }
    }

    pub fn push(&mut self, value: &str) {
        self.data.extend_from_slice(value.as_bytes());
        self.ends.push(self.data.len() as u32);
    }

    pub fn to_ref(&self) -> StrListRef<'_> {
        StrListRef {
            data: &self.data,
            ends: &self.ends,
        }
    }

    pub fn len(&self) -> usize {
        self.to_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.to_ref().is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.to_ref().get(index)
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &str {
        self.to_ref().get_unchecked(index)
    }

    pub fn iter(&self) -> StrListIter<'_> {
        self.to_ref().iter()
    }
}
