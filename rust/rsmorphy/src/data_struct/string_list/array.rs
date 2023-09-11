use crate::data_struct::string_list::array_ref::StringArrayRef;
use crate::data_struct::string_list::StringArrayBuilder;
use crate::data_struct::string_list::StringArrayIter;

#[derive(Debug, Clone)]
pub struct StringArray {
    pub(super) data: Box<[u8]>,
    pub(super) ends: Box<[u32]>,
}

impl StringArray {
    pub fn builder() -> StringArrayBuilder {
        StringArrayBuilder::new()
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

impl<'s> FromIterator<&'s str> for StringArray {
    fn from_iter<T: IntoIterator<Item = &'s str>>(iter: T) -> Self {
        let mut builder = StringArray::builder();
        for s in iter {
            builder.push(s);
        }
        builder.build()
    }
}
