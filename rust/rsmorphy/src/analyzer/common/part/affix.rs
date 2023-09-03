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
    pub affix: InlinableString,
    pub kind: AffixKind,
}

impl Affix {
    #[inline(always)]
    pub fn new(affix: impl Into<InlinableString>, kind: AffixKind) -> Self {
        let affix = affix.into();
        Affix { affix, kind }
    }

    #[inline(always)]
    pub fn known_suffix(affix: impl Into<InlinableString>) -> Self {
        Affix::new(affix, KnownSuffix)
    }

    #[inline(always)]
    pub fn known_prefix(affix: impl Into<InlinableString>) -> Self {
        Affix::new(affix, KnownPrefix)
    }

    #[inline(always)]
    pub fn unknown_prefix(affix: impl Into<InlinableString>) -> Self {
        Affix::new(affix, UnknownPrefix)
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
}
