use crate::collections::paradigm::ParadigmEntry;
use crate::collections::paradigm::ParadigmList;
use crate::cryostasis::Revive;

#[derive(Debug, Clone)]
pub struct ParadigmListSuspended {
    pub(super) idx: *const [u32],
    pub(super) data: *const [ParadigmEntry],
}

impl Revive for ParadigmListSuspended {
    type Revived<'d> = ParadigmList<'d>;

    unsafe fn revive<'d>(&self, _source_bin: &'d [u8]) -> Self::Revived<'d> {
        ParadigmList::new(unsafe { &*self.idx }, unsafe { &*self.data })
    }
}
