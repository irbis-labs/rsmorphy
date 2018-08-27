use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use analyzer::MorphAnalyzer;
use opencorpora::grammeme::GrammemeSet;
use opencorpora::kind::*;

//#[derive(Deserialize)]
#[derive(Debug, Clone, Eq)]
pub struct OpencorporaTagReg {
    pub string: String,
    pub grammemes: GrammemeSet,

    pub pos: Option<PartOfSpeech>,
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
        self.string.hash(state)
    }
}

impl PartialEq for OpencorporaTagReg {
    fn eq(&self, other: &OpencorporaTagReg) -> bool {
        self.string.eq(&other.string)
    }
}

impl OpencorporaTagReg {
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        let string = s.into();

        let grammemes = GrammemeSet::new(&string);

        let pos = PartOfSpeech::try_from_str(&string);
        let animacy = Animacy::try_from_str(&string);
        let aspect = Aspect::try_from_str(&string);
        let case = Case::try_from_str(&string);
        let gender = Gender::try_from_str(&string);
        let involvement = Involvement::try_from_str(&string);
        let mood = Mood::try_from_str(&string);
        let number = Number::try_from_str(&string);
        let person = Person::try_from_str(&string);
        let tense = Tense::try_from_str(&string);
        let transitivity = Transitivity::try_from_str(&string);
        let voice = Voice::try_from_str(&string);

        let has_apro = string.contains("Apro");

        OpencorporaTagReg {
            string,
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
            let meta = &morph.dict.grammeme_metas[grammeme];
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
            Some(PartOfSpeech::Noun) | Some(PartOfSpeech::Adjf) | Some(PartOfSpeech::Prtf) => {
                match self.pos {
                    Some(PartOfSpeech::Noun)
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

                    Some(PartOfSpeech::Noun) if index == 1 => {
                        Some((Number::Sing, Some(Case::Gent)))
                    }

                    Some(PartOfSpeech::Adjf) | Some(PartOfSpeech::Prtf)
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
                Some((number, None)) => hashset!{ number.to_grammeme() },
                Some((number, Some(case))) => hashset!{ number.to_grammeme(), case.to_grammeme() },
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
//            pos: Some(PartOfSpeech::Prtf),
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
