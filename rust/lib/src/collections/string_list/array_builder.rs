use crate::collections::string_list::array::StringArray;
use crate::collections::string_list::array_ref::StringArrayRef;
use crate::collections::string_list::StringArrayIter;

#[derive(Debug, Default)]
pub struct StringArrayBuilder {
    pub(super) data: Vec<u8>,
    pub(super) ends: Vec<u32>,
}

impl StringArrayBuilder {
    pub fn new() -> Self {
        StringArrayBuilder::default()
    }

    pub fn build(self) -> StringArray {
        StringArray {
            data: self.data.into_boxed_slice(),
            ends: self.ends.into_boxed_slice(),
        }
    }

    pub fn push(&mut self, value: &str) {
        self.data.extend_from_slice(value.as_bytes());
        self.ends.push(self.data.len() as u32);
    }

    pub fn as_ref(&self) -> StringArrayRef<'_> {
        StringArrayRef {
            data: &self.data,
            ends: &self.ends,
        }
    }

    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.as_ref().get(index)
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &str {
        self.as_ref().get_unchecked(index)
    }

    pub fn iter(&self) -> StringArrayIter<'_> {
        self.as_ref().iter()
    }
}
