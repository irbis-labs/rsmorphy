use crate::types::gram::GramId;

/// Залог
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Voice {
    /// действительный залог
    Actv,
    /// страдательный залог
    Pssv,
}

impl Voice {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Voice::*;

        Some(match id.as_str() {
            "actv" => Actv,
            "pssv" => Pssv,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            Voice::Actv => "actv",
            Voice::Pssv => "pssv",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Voice::Actv => "действительный залог",
            Voice::Pssv => "страдательный залог",
        }
    }
}
