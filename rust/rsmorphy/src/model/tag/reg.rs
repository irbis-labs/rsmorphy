use std::hash::Hash;
use std::hash::Hasher;

use string_cache::DefaultAtom;

use crate::model::gram::GramRegistry;
use crate::model::gram::GramSet;

#[derive(Debug, Clone)]
pub struct TagReg {
    pub fmt_int: DefaultAtom,
    pub grammemes: GramSet,
}

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
        let mut grammemes = GramSet::empty();

        #[cfg(feature = "trace_opencorpora")]
        log::trace!("tags for {}", fmt_int);

        for gram_int in fmt_int.split(|c| matches!(c, ',' | ' ')) {
            let Some(gram) = gram_registry.find_gram(gram_int) else {
                panic!("Unknown gram: {}", gram_int);
            };
            grammemes.insert(gram);
        }

        TagReg { fmt_int, grammemes }
    }
}

// fn numeral_match(
//     pos: Option<PartOfSpeech>,
//     case: Option<Case>,
//     gender: Option<Gender>,
//     num_kind: NumKind,
// ) -> Option<(Number, Option<Case>)> {
//     use PartOfSpeech as POS;
//
//     let pos = pos?;
//     let (POS::Noun | POS::AdjectiveFull | POS::ParticipleFull) = pos else {
//         return None;
//     };
//     let nomn_or_accs = matches!(case, Some(Case::Nomn | Case::Accs));
//
//     Some(match (pos, num_kind) {
//         (POS::Noun, _) if !nomn_or_accs => match num_kind {
//             NumKind::One => (Number::Sing, case),
//             _ => (Number::Plur, case),
//         },
//
//         (_, NumKind::One) => match case {
//             Some(Case::Nomn) => (Number::Sing, Some(Case::Nomn)),
//             _ => (Number::Sing, Some(Case::Accs)),
//         },
//
//         (POS::Noun, NumKind::Little) => (Number::Sing, Some(Case::Gent)),
//
//         (POS::AdjectiveFull | POS::ParticipleFull, NumKind::Little)
//             if gender == Some(Gender::Femn) =>
//         {
//             (Number::Plur, Some(Case::Nomn))
//         }
//
//         _ => (Number::Plur, Some(Case::Gent)),
//     })
// }
