use super::DictionaryMeta;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(64))]
pub struct DictionaryMetaOwned {
    pub max_suffix_length: u8,
    _reserved: [u8; 63],
}

impl DictionaryMetaOwned {
    #[inline(always)]
    pub fn new(max_suffix_length: u8) -> Self {
        Self {
            max_suffix_length,
            _reserved: [0; 63],
        }
    }

    #[inline(always)]
    pub fn get_view(&self) -> DictionaryMeta<'_> {
        DictionaryMeta::new(self)
    }
}
