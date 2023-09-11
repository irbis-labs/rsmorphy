use std::io;

use crate::data_struct::dawg::DawgValue;
use crate::data_struct::dawg::HH;
use crate::model::dictionary::ParadigmId;
use crate::model::dictionary::ParadigmIndex;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct WordValue {
    pub para_id: ParadigmId,
    pub idx: ParadigmIndex,
}

impl DawgValue for WordValue {
    #[inline(always)]
    fn new_in_place<F>(f: F) -> io::Result<Self>
    where
        F: FnOnce(&mut [u8]) -> io::Result<()>,
        Self: Sized,
    {
        let HH(para_id, idx) = HH::new_in_place(f)?;
        let para_id = ParadigmId::new(para_id);
        let idx = ParadigmIndex::new(idx);
        Ok(WordValue { para_id, idx })
    }
}
