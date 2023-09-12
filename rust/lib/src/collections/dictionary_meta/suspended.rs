use super::DictionaryMeta;
use super::DictionaryMetaOwned;
use crate::cryostasis::Revive;

#[derive(Debug, Clone, Copy)]
pub struct DictionaryMetaSuspended {
    pub(super) meta: *const DictionaryMetaOwned,
}

impl Revive for DictionaryMetaSuspended {
    type Revived<'a> = DictionaryMeta<'a>;

    unsafe fn revive<'a>(&self, _source_bin: &'a [u8]) -> DictionaryMeta<'a> {
        DictionaryMeta {
            meta: unsafe { &*self.meta },
        }
    }
}
