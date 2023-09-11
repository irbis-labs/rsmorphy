use super::DictionaryMetaOwned;
use super::DictionaryMetaSuspended;
use crate::model::dictionary::data_format::binpack::Suspend;

#[derive(Debug, Clone, Copy)]
pub struct DictionaryMeta<'a> {
    pub(super) meta: &'a DictionaryMetaOwned,
}

impl<'a> DictionaryMeta<'a> {
    #[inline(always)]
    pub fn new(meta: &'a DictionaryMetaOwned) -> Self {
        Self { meta }
    }

    #[inline(always)]
    pub fn into_inner(self) -> &'a DictionaryMetaOwned {
        self.meta
    }

    #[inline(always)]
    pub fn max_suffix_length(&self) -> usize {
        self.meta.max_suffix_length as usize
    }
}

impl Suspend for DictionaryMeta<'_> {
    type Suspended = DictionaryMetaSuspended;

    fn suspend(&self) -> Self::Suspended {
        DictionaryMetaSuspended {
            meta: self.meta as *const _,
        }
    }
}
