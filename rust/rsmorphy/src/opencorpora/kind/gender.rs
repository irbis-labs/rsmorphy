use crate::opencorpora::gram::GramId;

/// Род
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gender {
    /// мужской род
    Masc,
    /// женский род
    Femn,
    /// средний род
    Neut,
}

impl Gender {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Gender::*;

        Some(match id.as_str() {
            "masc" => Masc,
            "femn" => Femn,
            "neut" => Neut,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            Gender::Masc => "masc",
            Gender::Femn => "femn",
            Gender::Neut => "neut",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Gender::Masc => "мужской род",
            Gender::Femn => "женский род",
            Gender::Neut => "средний род",
        }
    }
}
