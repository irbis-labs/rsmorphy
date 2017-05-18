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
            AffixKind::KnownSuffix      => true,
            AffixKind::KnownPrefix      => true,
            AffixKind::UnknownPrefix    => false,
        }
    }
}
