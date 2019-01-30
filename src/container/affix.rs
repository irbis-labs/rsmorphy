use self::AffixKind::*;

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
        Affix::new(part, KnownSuffix)
    }

    pub fn known_prefix<S>(part: S) -> Self
    where
        S: Into<String>,
    {
        Affix::new(part, KnownPrefix)
    }

    pub fn unknown_prefix<S>(part: S) -> Self
    where
        S: Into<String>,
    {
        Affix::new(part, UnknownPrefix)
    }

    pub fn is_known(&self) -> bool {
        match self.kind {
            KnownSuffix | KnownPrefix => true,
            UnknownPrefix => false,
        }
    }

    pub fn is_known_suffix(&self) -> bool {
        match self.kind {
            KnownSuffix => true,
            _ => false,
        }
    }

    pub fn is_known_prefix(&self) -> bool {
        match self.kind {
            KnownPrefix => true,
            _ => false,
        }
    }

    pub fn is_unknown_prefix(&self) -> bool {
        match self.kind {
            UnknownPrefix => true,
            _ => false,
        }
    }

    pub fn is_prefix(&self) -> bool {
        match self.kind {
            KnownPrefix | UnknownPrefix => true,
            _ => false,
        }
    }

    pub fn is_suffix(&self) -> bool {
        match self.kind {
            KnownPrefix | UnknownPrefix => false,
            _ => true,
        }
    }

    pub fn title_rus(&self) -> &'static str {
        match self.kind {
            KnownPrefix => "Известный префикс",
            KnownSuffix => "Известный суффикс",
            UnknownPrefix => "Неизвестный префикс",
        }
    }
}
