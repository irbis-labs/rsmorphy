use crate::model::dictionary::data_format::binpack::Revive;
use crate::model::dictionary::ParadigmEntry;
use crate::model::ParadigmList;

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
