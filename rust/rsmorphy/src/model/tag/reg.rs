use std::hash::Hash;
use std::hash::Hasher;
use std::mem;

use byteorder::ByteOrder;

use crate::data_struct::pascal_string::PascalString;
use crate::model::gram::GramRegistry;
use crate::model::gram::GramSet;

const TAG_REG_SIZE: usize = 128;
const FMT_INT_MAX_LEN: usize = TAG_REG_SIZE - mem::size_of::<GramSet>() - 1;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(64))]
pub struct TagReg {
    pub fmt_int: PascalString<FMT_INT_MAX_LEN>,
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
    pub fn from_fmt_int(gram_registry: GramRegistry<'_>, s: impl AsRef<str>) -> Self {
        Self::from_fmt_int_(gram_registry, s.as_ref())
    }

    pub fn from_fmt_int_(gram_registry: GramRegistry<'_>, fmt_int: &str) -> Self {
        let mut grammemes = GramSet::empty();

        #[cfg(feature = "trace_model")]
        log::trace!("tags for {}", fmt_int);

        for gram_int in fmt_int.split(|c| matches!(c, ',' | ' ')) {
            let Some(gram) = gram_registry.find_gram(gram_int) else {
                panic!("Unknown gram: {}", gram_int);
            };
            grammemes.insert(gram);
        }

        let Ok(fmt_int) = PascalString::try_from(fmt_int) else {
            panic!(
                "Too long tag {} ({} exceeds {} bytes)",
                fmt_int,
                fmt_int.len(),
                FMT_INT_MAX_LEN
            );
        };

        TagReg { fmt_int, grammemes }
    }

    pub fn into_order<Order: ByteOrder>(self) -> Self {
        TagReg {
            fmt_int: self.fmt_int,
            grammemes: self.grammemes.into_order::<Order>(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(mem::size_of::<TagReg>(), TAG_REG_SIZE);
    }
}
