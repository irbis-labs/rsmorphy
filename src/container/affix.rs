#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AffixKind {
    KnownSuffix,
    KnownPrefix,
    UnknownPrefix,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Affix {
    pub part: String,
    pub kind: AffixKind,
}


impl Affix {
    pub fn is_known(&self) -> bool {
        match self.kind {
            AffixKind::KnownSuffix |
            AffixKind::KnownPrefix      => true,
            AffixKind::UnknownPrefix    => false,
        }
    }

    pub fn is_known_suffix(&self) -> bool {
        match self.kind {
            AffixKind::KnownSuffix => true,
            _ => false,
        }
    }

    pub fn is_known_prefix(&self) -> bool {
        match self.kind {
            AffixKind::KnownPrefix => true,
            _ => false,
        }
    }

    pub fn is_unknown_prefix(&self) -> bool {
        match self.kind {
            AffixKind::UnknownPrefix => true,
            _ => false,
        }
    }
}
