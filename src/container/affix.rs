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
    pub fn new<S>(part: S, kind: AffixKind) -> Self
    where
        S: Into<String>,
    {
        let part = part.into();
        Affix { part, kind }
    }

    pub fn known_suffix<S>(part: S) -> Self
    where
        S: Into<String>,
    {
        Affix::new(part, AffixKind::KnownSuffix)
    }

    pub fn known_prefix<S>(part: S) -> Self
    where
        S: Into<String>,
    {
        Affix::new(part, AffixKind::KnownPrefix)
    }

    pub fn unknown_prefix<S>(part: S) -> Self
    where
        S: Into<String>,
    {
        Affix::new(part, AffixKind::UnknownPrefix)
    }

    pub fn is_known(&self) -> bool {
        match self.kind {
            AffixKind::KnownSuffix | AffixKind::KnownPrefix => true,
            AffixKind::UnknownPrefix => false,
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
