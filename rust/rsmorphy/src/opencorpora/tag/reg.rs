use std::hash::Hash;
use std::hash::Hasher;

use serde::Deserialize;
use serde::Serialize;
use smallvec::SmallVec;
use string_cache::DefaultAtom;

use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::gram::GramRegistry;
use crate::opencorpora::gram::GramSet;
use crate::opencorpora::kind::*;
use crate::opencorpora::tag::set::TagSet;

#[derive(Serialize, Deserialize)]
//
#[derive(Debug, Clone)]
pub struct TagReg {
    pub fmt_int: DefaultAtom,
    pub grammemes: GramSet,
    pub tags: TagSet,
}

// impl Serialize for TagReg {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let string: &str = self.fmt_int.as_ref();
//         string.serialize(serializer)
//     }
// }
//
// impl<'de> Deserialize<'de> for TagReg {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let string: String = Deserialize::deserialize(deserializer)?;
//         Ok(TagReg::from_fmt_int(string))
//     }
// }

impl Hash for TagReg {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.grammemes.hash(state)
    }
}

impl PartialEq for TagReg {
    fn eq(&self, other: &TagReg) -> bool {
        self.grammemes.eq(&other.grammemes)
    }
}

impl Eq for TagReg {}

impl TagReg {
    #[inline(always)]
    pub fn from_fmt_int(gram_registry: &GramRegistry, s: impl Into<DefaultAtom>) -> Self {
        Self::from_fmt_int_(gram_registry, s.into())
    }

    pub fn from_fmt_int_(gram_registry: &GramRegistry, fmt_int: DefaultAtom) -> Self {
        let mut grammemes = GramSet::new();
        let mut tags = TagSet::new();
        // FIXME: dig from where this "anim"-"inan" conflict comes from.
        let fmt_int_workaround = match fmt_int.as_ref() {
            "NOUN,anim,masc,Inmx sing,accs,inan" => "NOUN,anim,masc,Inmx sing,accs",
            "NOUN,anim,masc,Inmx plur,accs,inan" => "NOUN,anim,masc,Inmx plur,accs",
            "NOUN,inan,femn,Inmx anim,plur,accs" => "NOUN,inan,femn,Inmx plur,accs",
            "NOUN,anim,neut,Inmx plur,accs,inan" => "NOUN,anim,neut,Inmx plur,accs",
            other => other,
        };
        #[cfg(feature = "trace_opencorpora")]
        log::trace!("tags for {}", fmt_int);
        for gram_int in fmt_int_workaround.split(|c| matches!(c, ',' | ' ')) {
            let Some(gram) = gram_registry.find_gram(gram_int) else {
                panic!("Unknown gram: {}", gram_int);
            };
            grammemes.insert(gram);
            let gram_reg = &gram_registry.get_reg_by_gram(gram);

            #[cfg(feature = "trace_opencorpora")]
            log::trace!("gram tags: {:?}, {:?}", gram_reg.name, gram_reg.tag);
            #[cfg(feature = "trace_opencorpora")]
            log::trace!("tag_reg tags {:?}", tags);

            tags = tags.join(gram_reg.tag).unwrap();
        }

        TagReg {
            fmt_int,
            grammemes,
            tags,
        }
    }

    #[inline(always)]
    pub fn prepare_required(&self, morph: &MorphAnalyzer, required: GramSet) -> GramSet {
        self.prepare_required_(&morph.dict().grammemes, required)
    }

    #[inline(always)]
    fn prepare_required_(&self, registry: &GramRegistry, required: GramSet) -> GramSet {
        required
            .iter()
            .fold(self.grammemes | required, |acc, gram| {
                acc - registry.get_reg_by_gram(gram).incompatible
            })
    }

    /// Returns grammemes that are required to be present in the word.
    /// The number should be passed similar to `(number.abs() % 100) as u8`.
    pub fn numeral_agreement_grammemes(&self, morph: &MorphAnalyzer, num: u8) -> GramSet {
        self.numeral_agreement_grammemes_(&morph.dict().grammemes, num)
    }

    pub fn numeral_agreement_grammemes_(&self, registry: &GramRegistry, num: u8) -> GramSet {
        let (num_10, num_100) = (num % 10, num % 100);
        let num_kind = match () {
            () if (num_10 == 1) && (num_100 != 11) => NumKind::One,
            () if (num_10 >= 2) && (num_10 <= 4) && (num_100 < 10 || num_100 >= 20) => {
                NumKind::Little
            }
            () => NumKind::Many,
        };

        let tags = &self.tags;
        let mut grams: SmallVec<[&str; 2]> = SmallVec::new();
        match numeral_match(tags.pos, tags.case, tags.gender, num_kind) {
            None => (),
            Some((number, case)) => {
                grams.push(number.gram_int());
                match case {
                    None => {}
                    Some(case) => {
                        grams.push(case.gram_int());
                    }
                }
            }
        };
        grams
            .iter()
            .copied()
            // SAFETY: all tags represented by enum variants should be present in the registry.
            .map(|gram_int| registry.find_gram(gram_int).unwrap())
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
enum NumKind {
    One,
    Little,
    Many,
}

fn numeral_match(
    pos: Option<PartOfSpeech>,
    case: Option<Case>,
    gender: Option<Gender>,
    num_kind: NumKind,
) -> Option<(Number, Option<Case>)> {
    use PartOfSpeech as POS;

    let pos = pos?;
    let (POS::Noun | POS::AdjectiveFull | POS::ParticipleFull) = pos else {
        return None;
    };
    let nomn_or_accs = matches!(case, Some(Case::Nomn | Case::Accs));

    Some(match (pos, num_kind) {
        (POS::Noun, _) if !nomn_or_accs => match num_kind {
            NumKind::One => (Number::Sing, case),
            _ => (Number::Plur, case),
        },

        (_, NumKind::One) => match case {
            Some(Case::Nomn) => (Number::Sing, Some(Case::Nomn)),
            _ => (Number::Sing, Some(Case::Accs)),
        },

        (POS::Noun, NumKind::Little) => (Number::Sing, Some(Case::Gent)),

        (POS::AdjectiveFull | POS::ParticipleFull, NumKind::Little)
            if gender == Some(Gender::Femn) =>
        {
            (Number::Plur, Some(Case::Nomn))
        }
        _ => (Number::Plur, Some(Case::Gent)),
    })
}
