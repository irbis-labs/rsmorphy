use std::io;

use crate::collections::dawg::DawgValue;
use crate::collections::dawg::HHH;
use crate::collections::paradigm::ParadigmId;
use crate::collections::paradigm::ParadigmIndex;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct PredictionSuffixValue {
    pub count: u16,
    pub para_id: ParadigmId,
    pub idx: ParadigmIndex,
}

impl DawgValue for PredictionSuffixValue {
    #[inline(always)]
    fn new_in_place<F>(f: F) -> io::Result<Self>
    where
        F: FnOnce(&mut [u8]) -> io::Result<()>,
        Self: Sized,
    {
        let HHH(count, para_id, idx) = HHH::new_in_place(f)?;
        let para_id = ParadigmId::new(para_id);
        let idx = ParadigmIndex::new(idx);
        Ok(PredictionSuffixValue {
            count,
            para_id,
            idx,
        })
    }
}
