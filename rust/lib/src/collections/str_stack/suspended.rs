use crate::collections::str_stack::StrListRef;
use crate::cryostasis::Revive;

#[derive(Debug, Clone)]
pub struct StrListRefSuspended {
    pub(super) data: *const [u8],
    pub(super) ends: *const [u32],
}

impl Revive for StrListRefSuspended {
    type Revived<'a> = StrListRef<'a>;

    unsafe fn revive<'a>(&self, _source_bin: &'a [u8]) -> StrListRef<'a> {
        StrListRef {
            data: unsafe { &*self.data },
            ends: unsafe { &*self.ends },
        }
    }
}
