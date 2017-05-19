use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

use serde_json::Value;


use analyzer::MorphAnalyzer;
use opencorpora::grammeme::GrammemeSet;
use opencorpora::kind::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpencorporaTagReg {
    pub string:         String,
    pub grammemes:      GrammemeSet,

    pub pos:            Option<PartOfSpeach>,
    pub animacy:        Option<Animacy>,
    pub aspect:         Option<Aspect>,
    pub case:           Option<Case>,
    pub gender:         Option<Gender>,
    pub involvement:    Option<Involvement>,
    pub mood:           Option<Mood>,
    pub number:         Option<Number>,
    pub person:         Option<Person>,
    pub tense:          Option<Tense>,
    pub transitivity:   Option<Transitivity>,
    pub voice:          Option<Voice>,

    pub has_apro:       bool,
}


impl Hash for OpencorporaTagReg {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string.hash(state)
    }
}


impl OpencorporaTagReg {
    pub fn from_str<'a, P>(s: P) -> Self where P: Into<&'a str> {
        let s = s.into();
        OpencorporaTagReg {
            string: s.to_owned(),
            grammemes: GrammemeSet::from_str(s),

            pos: PartOfSpeach::try_from_str(s),
            animacy: Animacy::try_from_str(s),
            aspect: Aspect::try_from_str(s),
            case: Case::try_from_str(s),
            gender: Gender::try_from_str(s),
            involvement: Involvement::try_from_str(s),
            mood: Mood::try_from_str(s),
            number: Number::try_from_str(s),
            person: Person::try_from_str(s),
            tense: Tense::try_from_str(s),
            transitivity: Transitivity::try_from_str(s),
            voice: Voice::try_from_str(s),

            has_apro: s.contains("Apro"),
        }
    }

    pub fn vec_from_json(data: Value) -> Vec<Self> {
        let data = data.as_array().unwrap();
        Vec::from_iter(data.into_iter().map(
            |v| OpencorporaTagReg::from_str(v.as_str().unwrap())
        ))
    }

    pub fn is_productive(&self) -> bool {
        self.pos
            .map(|pos| pos.is_productive())
            .unwrap_or_else(|| !self.has_apro )
    }

    pub fn prepare_required(&self, morph: &MorphAnalyzer, required: &GrammemeSet) -> GrammemeSet {
        let mut new_grammemes = self.grammemes.set.clone();
        new_grammemes.extend(required.set.iter().cloned());
        for grammeme in &required.set {
            let meta = morph.dict.grammeme_metas.get(grammeme).unwrap();
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
            Some(PartOfSpeach::Noun) |
            Some(PartOfSpeach::Adjf) |
            Some(PartOfSpeach::Prtf) => match self.pos {

                Some(PartOfSpeach::Noun)
                    if self.case != Some(Case::Nomn) && self.case != Some(Case::Accs)
                                => match self.case {

                    _ if index == 0 => Some((Number::Sing, self.case)),
                    _               => Some((Number::Plur, self.case)),
                },

                _ if index == 0 => match self.case {

                    Some(Case::Nomn) => Some((Number::Sing, Some(Case::Nomn))),
                    _                => Some((Number::Sing, Some(Case::Accs)))
                },

                Some(PartOfSpeach::Noun) if index == 1
                                => Some((Number::Sing, Some(Case::Gent))),

                Some(PartOfSpeach::Adjf) | Some(PartOfSpeach::Prtf)
                    if index == 1 && self.gender == Some(Gender::Femn)
                                => Some((Number::Plur, Some(Case::Nomn))),

                _               => Some((Number::Plur, Some(Case::Gent))),
            },
            _ => None
        };

        GrammemeSet {
            set: match x {
                None                        => Default::default(),
                Some((number, None))        => hashset!{ number.to_grammeme() },
                Some((number, Some(case)))  => hashset!{ number.to_grammeme(), case.to_grammeme() },
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
