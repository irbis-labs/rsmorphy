use crate::GramSet;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct NumeralGrammemes {
    pub noun: GramSet,
    pub adjf: GramSet,
    pub prtf: GramSet,
    pub sing: GramSet,
    pub plur: GramSet,
    pub nomn: GramSet,
    pub gent: GramSet,
    pub accs: GramSet,
    pub femn: GramSet,
    pub case: GramSet,
}
