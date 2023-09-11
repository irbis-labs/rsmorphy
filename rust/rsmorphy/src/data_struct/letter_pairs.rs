use crate::model::dictionary::data_format::binpack::Revive;
use crate::model::dictionary::data_format::binpack::Suspend;

#[derive(Debug, Clone, Copy)]
pub struct LetterPairs<'d> {
    chars: &'d [char],
    subst: &'d [char],
}

#[derive(Debug, Clone)]
pub struct LetterPairsSuspended {
    chars: *const [char],
    subst: *const [char],
}

impl<'a> LetterPairs<'a> {
    pub const fn new(chars: &'a [char], subst: &'a [char]) -> Self {
        assert!(chars.len() == subst.len());
        LetterPairs { chars, subst }
    }

    pub const fn into_inner(self) -> (&'a [char], &'a [char]) {
        (self.chars, self.subst)
    }

    pub fn find_pair(&self, letter: &str) -> Option<char> {
        let mut it = letter.chars();

        let Some(letter) = it.next() else { None? };

        if it.next().is_some() {
            None?
        }

        let pos = self.chars.iter().copied().position(|c| c == letter)?;
        Some(self.subst[pos])
    }
}

impl<'d> Suspend for LetterPairs<'d> {
    type Suspended = LetterPairsSuspended;

    fn suspend(&self) -> Self::Suspended {
        LetterPairsSuspended {
            chars: self.chars as *const _,
            subst: self.subst as *const _,
        }
    }
}

impl Revive for LetterPairsSuspended {
    type Revived<'d> = LetterPairs<'d>;

    unsafe fn revive<'d>(&self, _source_bin: &'d [u8]) -> Self::Revived<'d> {
        LetterPairs::new(unsafe { &*self.chars }, unsafe { &*self.subst })
    }
}
