use crate::cryostasis::Revive;
use crate::cryostasis::Suspend;
use crate::types::gram::Gram;

#[derive(Debug, Clone, Copy)]
pub struct GramPairs<'d> {
    replaces: &'d [(Gram, Gram)],
}

#[derive(Debug, Clone)]
pub struct GramPairsSuspended {
    replaces: *const [(Gram, Gram)],
}

impl<'a> GramPairs<'a> {
    pub const fn new(replaces: &'a [(Gram, Gram)]) -> Self {
        GramPairs { replaces }
    }

    pub const fn into_inner(self) -> &'a [(Gram, Gram)] {
        self.replaces
    }

    pub fn iter(&self) -> impl Iterator<Item = (Gram, Gram)> + 'a {
        self.replaces.iter().copied()
    }
}

impl<'d> Suspend for GramPairs<'d> {
    type Suspended = GramPairsSuspended;

    fn suspend(&self) -> Self::Suspended {
        GramPairsSuspended {
            replaces: self.replaces as *const _,
        }
    }
}

impl Revive for GramPairsSuspended {
    type Revived<'d> = GramPairs<'d>;

    unsafe fn revive<'d>(&self, _source_bin: &'d [u8]) -> Self::Revived<'d> {
        GramPairs::new(unsafe { &*self.replaces })
    }
}
