use crate::collections::string_list::StringArrayRef;
use crate::cryostasis::Revive;

#[derive(Debug, Clone)]
pub struct StringArraySuspended {
    pub(super) data: *const [u8],
    pub(super) ends: *const [u32],
}

impl Revive for StringArraySuspended {
    type Revived<'a> = StringArrayRef<'a>;

    unsafe fn revive<'a>(&self, _source_bin: &'a [u8]) -> StringArrayRef<'a> {
        StringArrayRef {
            data: unsafe { &*self.data },
            ends: unsafe { &*self.ends },
        }
    }
}
