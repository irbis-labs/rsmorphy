use crate::types::gram::GramId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mood {
    /// изъявительное наклонение
    Indc,
    /// повелительное наклонение
    Impr,
}

impl Mood {
    pub fn gram_int(self) -> &'static str {
        match self {
            Mood::Indc => "indc",
            Mood::Impr => "impr",
        }
    }

    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Mood::*;

        Some(match id.as_str() {
            "indc" => Indc,
            "impr" => Impr,
            _ => None?,
        })
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Mood::Indc => "изъявительное наклонение",
            Mood::Impr => "повелительное наклонение",
        }
    }
}
