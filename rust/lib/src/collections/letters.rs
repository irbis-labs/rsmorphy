use crate::cryostasis::Revive;
use crate::cryostasis::Suspend;

#[derive(Debug, Clone, Copy)]
pub struct Letters<'d> {
    chars: &'d [char],
}

#[derive(Debug, Clone)]
pub struct LettersSuspended {
    chars: *const [char],
}

impl<'a> Letters<'a> {
    pub const fn new(chars: &'a [char]) -> Self {
        Letters { chars }
    }

    pub const fn into_inner(self) -> &'a [char] {
        self.chars
    }

    pub fn find(&self, letter: &str) -> Option<char> {
        let mut it = letter.chars();

        let Some(letter) = it.next() else { None? };

        if it.next().is_some() {
            None?
        }

        self.chars.iter().copied().find(|&c| c == letter)
    }
}

impl<'d> Suspend for Letters<'d> {
    type Suspended = LettersSuspended;

    fn suspend(&self) -> Self::Suspended {
        LettersSuspended {
            chars: self.chars as *const _,
        }
    }
}

impl Revive for LettersSuspended {
    type Revived<'d> = Letters<'d>;

    unsafe fn revive<'d>(&self, _source_bin: &'d [u8]) -> Self::Revived<'d> {
        Letters::new(unsafe { &*self.chars })
    }
}
