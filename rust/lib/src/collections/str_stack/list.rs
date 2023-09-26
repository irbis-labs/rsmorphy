use crate::collections::str_stack::ref_::StrListRef;
use crate::collections::str_stack::StrListIter;
use crate::collections::str_stack::StrStack;

#[derive(Debug, Clone)]
pub struct StrList {
    pub(super) data: Box<[u8]>,
    pub(super) ends: Box<[u32]>,
}

impl StrList {
    pub fn builder() -> StrStack {
        StrStack::new()
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

impl<'s> FromIterator<&'s str> for StrList {
    fn from_iter<T: IntoIterator<Item = &'s str>>(iter: T) -> Self {
        let mut builder = StrList::builder();
        for s in iter {
            builder.push(s);
        }
        builder.build()
    }
}
