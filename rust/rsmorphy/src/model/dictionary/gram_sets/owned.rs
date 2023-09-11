use super::GramSets;
use crate::model::gram::GramRegistry;
use crate::GramSet;

/// Number of grammatical sets in NumeralGrammemes
pub(super) const NUMERAL_GRAMSETS_LEN: usize = 10;
pub(super) const NUMBER_OF_GRAM_SETS: usize = NUMERAL_GRAMSETS_LEN + 1 + 1;
pub(super) const NON_PORDUCTIVE_GRAMSET_INDEX: usize = NUMERAL_GRAMSETS_LEN;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(64))]
pub struct GramSetsOwned {
    pub(super) sets: [GramSet; NUMBER_OF_GRAM_SETS],
}

impl GramSetsOwned {
    pub fn new(registry: GramRegistry<'_>, non_productive: GramSet) -> Self {
        let noun = registry.find_gram("NOUN").unwrap().as_set();
        let adjf = registry.find_gram("ADJF").unwrap().as_set();
        let prtf = registry.find_gram("PRTF").unwrap().as_set();

        let sing = registry.find_gram("sing").unwrap().as_set();
        let plur = registry.find_gram("plur").unwrap().as_set();

        let nomn = registry.find_gram("nomn").unwrap().as_set();
        let gent = registry.find_gram("gent").unwrap().as_set();
        let accs = registry.find_gram("accs").unwrap().as_set();

        let femn = registry.find_gram("femn").unwrap().as_set();

        let case = Some(registry.find_gram("CAse").unwrap());
        let case = registry
            .iter()
            .filter(|gram_reg| gram_reg.parent == case)
            .map(|gram_reg| gram_reg.gram)
            .collect();

        let reserved = GramSet::empty();

        let sets = [
            noun,
            adjf,
            prtf,
            sing,
            plur,
            nomn,
            gent,
            accs,
            femn,
            case,
            non_productive,
            reserved,
        ];
        GramSetsOwned { sets }
    }

    pub fn get_view(&self) -> GramSets<'_> {
        let sets = self;
        GramSets { sets }
    }
}
