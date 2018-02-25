use std::iter::FromIterator;


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ParadigmEntry {
    pub prefix_id: u16,
    pub tag_id: u16,
    pub suffix_id: u16,
}

impl ParadigmEntry {
    pub fn build(paradigm: &[u16]) -> Vec<Self> {
        assert_eq!(0, paradigm.len() % 3, "Wrong paradigm length");
        let paradigm_len = paradigm.len() / 3;
        Vec::from_iter((0..paradigm_len).map(
            |idx| ParadigmEntry {
                suffix_id: paradigm[idx],
                tag_id: paradigm[paradigm_len + idx],
                prefix_id: paradigm[paradigm_len * 2 + idx],
            }
        ))
    }
}
