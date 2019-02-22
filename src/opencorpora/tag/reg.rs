use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use maplit::hashset;
use string_cache::DefaultAtom;

use crate::{
    analyzer::MorphAnalyzer,
    opencorpora::{grammeme::GrammemeSet, kind::*},
};

//#[derive(Deserialize)]
#[derive(Debug, Clone, Eq)]
pub struct OpencorporaTagReg {
    pub fmt_int: DefaultAtom,
    pub grammemes: GrammemeSet,

    pub pos: Option<PartOfSpeach>,
    pub animacy: Option<Animacy>,
    pub aspect: Option<Aspect>,
    pub case: Option<Case>,
    pub gender: Option<Gender>,
    pub involvement: Option<Involvement>,
    pub mood: Option<Mood>,
    pub number: Option<Number>,
    pub person: Option<Person>,
    pub tense: Option<Tense>,
    pub transitivity: Option<Transitivity>,
    pub voice: Option<Voice>,

    pub has_apro: bool,
}

impl Hash for OpencorporaTagReg {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fmt_int.hash(state)
    }
}

impl PartialEq for OpencorporaTagReg {
    fn eq(&self, other: &OpencorporaTagReg) -> bool {
        self.fmt_int.eq(&other.fmt_int)
    }
}

impl OpencorporaTagReg {
    pub fn new<S>(s: S) -> Self
    where
        S: Into<DefaultAtom>,
    {
        let fmt_int = s.into();

        let grammemes = GrammemeSet::parse_fmt_int(&fmt_int);

        let pos = PartOfSpeach::try_from_fmt_int(&fmt_int);
        let animacy = Animacy::try_from_fmt_int(&fmt_int);
        let aspect = Aspect::try_from_fmt_int(&fmt_int);
        let case = Case::try_from_fmt_int(&fmt_int);
        let gender = Gender::try_from_fmt_int(&fmt_int);
        let involvement = Involvement::try_from_fmt_int(&fmt_int);
        let mood = Mood::try_from_fmt_int(&fmt_int);
        let number = Number::try_from_fmt_int(&fmt_int);
        let person = Person::try_from_fmt_int(&fmt_int);
        let tense = Tense::try_from_fmt_int(&fmt_int);
        let transitivity = Transitivity::try_from_fmt_int(&fmt_int);
        let voice = Voice::try_from_fmt_int(&fmt_int);

        let has_apro = fmt_int.contains("Apro");

        OpencorporaTagReg {
            fmt_int,
            grammemes,
            has_apro,
            pos,
            animacy,
            aspect,
            case,
            gender,
            involvement,
            mood,
            number,
            person,
            tense,
            transitivity,
            voice,
        }
    }

    pub fn is_productive(&self) -> bool {
        self.pos
            .map(|pos| pos.is_productive())
            .unwrap_or_else(|| !self.has_apro)
    }

    pub fn prepare_required(&self, morph: &MorphAnalyzer, required: &GrammemeSet) -> GrammemeSet {
        let mut new_grammemes = self.grammemes.set.clone();
        new_grammemes.extend(required.set.iter().cloned());
        for grammeme in &required.set {
            let meta = &morph.dict.grammemes[grammeme].1;
            new_grammemes = &new_grammemes - &meta.incompatible;
        }
        GrammemeSet { set: new_grammemes }
    }

    pub fn numeral_agreement_grammemes(&self, num: usize) -> GrammemeSet {
        let index = match num {
            num if (num % 10 == 1) && (num % 100 != 11) => 0,
            num if (num % 10 >= 2) && (num % 10 <= 4) && (num % 100 < 10 || num % 100 >= 20) => 1,
            _ => 2,
        };

        let x = match self.pos {
            Some(PartOfSpeach::Noun) | Some(PartOfSpeach::Adjf) | Some(PartOfSpeach::Prtf) => {
                match self.pos {
                    Some(PartOfSpeach::Noun)
                        if self.case != Some(Case::Nomn) && self.case != Some(Case::Accs) =>
                    {
                        match self.case {
                            _ if index == 0 => Some((Number::Sing, self.case)),
                            _ => Some((Number::Plur, self.case)),
                        }
                    }

                    _ if index == 0 => match self.case {
                        Some(Case::Nomn) => Some((Number::Sing, Some(Case::Nomn))),
                        _ => Some((Number::Sing, Some(Case::Accs))),
                    },

                    Some(PartOfSpeach::Noun) if index == 1 => {
                        Some((Number::Sing, Some(Case::Gent)))
                    }

                    Some(PartOfSpeach::Adjf) | Some(PartOfSpeach::Prtf)
                        if index == 1 && self.gender == Some(Gender::Femn) =>
                    {
                        Some((Number::Plur, Some(Case::Nomn)))
                    }

                    _ => Some((Number::Plur, Some(Case::Gent))),
                }
            }
            _ => None,
        };

        GrammemeSet {
            set: match x {
                None => HashSet::default(),
                Some((number, None)) => hashset! { number.to_grammeme() },
                Some((number, Some(case))) => {
                    maplit::hashset! { number.to_grammeme(), case.to_grammeme() }
                }
            },
        }
    }
}

//#[cfg(test)]
//mod tests {
//    use opencorpora::tag::OpencorporaTagReg;
//    use opencorpora::kind::*;
//
//    #[test]
//    fn from_str() {
//        let sample = OpencorporaTagReg::from_str("PRTF,impf,tran,past,actv anim,masc,sing,accs,Infr");
//        let tag = OpencorporaTagReg {
//            string: "PRTF,impf,tran,past,actv anim,masc,sing,accs,Infr".into(),
//            pos: Some(PartOfSpeach::Prtf),
//            animacy: Some(Animacy::Anim),
//            aspect: Some(Aspect::Impf),
//            case: Some(Case::Accs),
//            gender: Some(Gender::Masc),
//            involvement: None,
//            mood: None,
//            number: Some(Number::Sing),
//            person: None,
//            tense: Some(Tense::Past),
//            transitivity: Some(Transitivity::Tran),
//            voice: Some(Voice::Actv)
//        };
//        assert_eq!(tag, sample);
//    }
//}
