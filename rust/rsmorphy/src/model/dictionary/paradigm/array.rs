use crate::model::dictionary::data_format::binpack::Suspend;
use crate::model::dictionary::Paradigm;
use crate::model::dictionary::ParadigmEntry;
use crate::model::dictionary::ParadigmListSuspended;
use crate::model::ParadigmId;

#[derive(Debug, Clone, Copy)]
pub struct ParadigmList<'d> {
    idx: &'d [u32],
    data: &'d [ParadigmEntry],
}

impl<'d> ParadigmList<'d> {
    pub(crate) fn new(idx: &'d [u32], data: &'d [ParadigmEntry]) -> Self {
        ParadigmList { idx, data }
    }

    pub fn into_inner(self) -> (&'d [u32], &'d [ParadigmEntry]) {
        (self.idx, self.data)
    }

    #[inline]
    pub fn get(&self, id: ParadigmId) -> Option<Paradigm<'d>> {
        let idx = id.index();
        if idx + 1 < self.idx.len() {
            let (start, end) = (self.idx[idx] as usize, self.idx[idx + 1] as usize);
            Some(&self.data[start..end])
        } else {
            None
        }
    }
}

impl<'d> Suspend for ParadigmList<'d> {
    type Suspended = ParadigmListSuspended;

    fn suspend(&self) -> Self::Suspended {
        ParadigmListSuspended {
            idx: self.idx as *const [u32],
            data: self.data as *const [ParadigmEntry],
        }
    }
}
