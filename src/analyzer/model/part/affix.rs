use inlinable_string::InlinableString;

use self::AffixKind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AffixKind {
    KnownSuffix,
    KnownPrefix,
    UnknownPrefix,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Affix {
    pub(in crate::analyzer) part: InlinableString,
    pub(in crate::analyzer) kind: AffixKind,
}

impl Affix {
    #[inline(always)]
    pub fn new<S>(part: S, kind: AffixKind) -> Self
    where
        S: Into<InlinableString>,
    {
        let part = part.into();
        Affix { part, kind }
    }

    #[inline(always)]
    pub fn part(&self) -> &str {
        &self.part
    }

    #[inline(always)]
    pub fn kind(&self) -> AffixKind {
        self.kind
    }

    #[inline(always)]
    pub fn into_parts(self) -> (InlinableString, AffixKind) {
        (self.part, self.kind)
    }

    #[inline(always)]
    pub fn known_suffix<S>(part: S) -> Self
    where
        S: Into<InlinableString>,
    {
        Affix::new(part, KnownSuffix)
    }

    #[inline(always)]
    pub fn known_prefix<S>(part: S) -> Self
    where
        S: Into<InlinableString>,
    {
        Affix::new(part, KnownPrefix)
    }

    #[inline(always)]
    pub fn unknown_prefix<S>(part: S) -> Self
    where
        S: Into<InlinableString>,
    {
        Affix::new(part, UnknownPrefix)
    }

    #[inline(always)]
    pub fn is_known(&self) -> bool {
        matches!(self.kind, KnownSuffix | KnownPrefix)
    }

    #[inline(always)]
    pub fn is_known_suffix(&self) -> bool {
        matches!(self.kind, KnownSuffix)
    }

    #[inline(always)]
    pub fn is_known_prefix(&self) -> bool {
        matches!(self.kind, KnownPrefix)
    }

    #[inline(always)]
    pub fn is_unknown_prefix(&self) -> bool {
        matches!(self.kind, UnknownPrefix)
    }

    #[inline(always)]
    pub fn is_prefix(&self) -> bool {
        matches!(self.kind, KnownPrefix | UnknownPrefix)
    }

    #[inline(always)]
    pub fn is_suffix(&self) -> bool {
        matches!(self.kind, KnownSuffix)
    }

    #[inline(always)]
    pub fn title_rus(&self) -> &'static str {
        match self.kind {
            KnownPrefix => "Известный префикс",
            KnownSuffix => "Известный суффикс",
            UnknownPrefix => "Неизвестный префикс",
        }
    }
}
