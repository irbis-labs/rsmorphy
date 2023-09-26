use smart_string::SmartString;

use self::AffixKind::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AffixKind {
    KnownSuffix,
    KnownPrefix,
    UnknownPrefix,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Affix {
    pub affix: SmartString,
    pub kind: AffixKind,
}

impl Affix {
    #[inline(always)]
    pub fn new(affix: impl Into<SmartString>, kind: AffixKind) -> Self {
        let affix = affix.into();
        Affix { affix, kind }
    }

    #[inline(always)]
    pub fn known_suffix(affix: impl Into<SmartString>) -> Self {
        Affix::new(affix, KnownSuffix)
    }

    #[inline(always)]
    pub fn known_prefix(affix: impl Into<SmartString>) -> Self {
        Affix::new(affix, KnownPrefix)
    }

    #[inline(always)]
    pub fn unknown_prefix(affix: impl Into<SmartString>) -> Self {
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
