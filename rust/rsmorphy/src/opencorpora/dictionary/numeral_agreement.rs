use crate::opencorpora::gram::GramRegistry;
use crate::GramSet;

#[derive(Debug, Clone, Copy)]
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

impl NumeralGrammemes {
    pub fn new(registry: &GramRegistry) -> Self {
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

        NumeralGrammemes {
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
        }
    }
}
