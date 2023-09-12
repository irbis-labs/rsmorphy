use crate::types::gram::GramId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tense {
    /// настоящее время
    Pres,
    /// прошедшее время
    Past,
    /// будущее время
    Futr,
}

impl Tense {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Tense::*;

        Some(match id.as_str() {
            "pres" => Pres,
            "past" => Past,
            "futr" => Futr,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            Tense::Pres => "pres",
            Tense::Past => "past",
            Tense::Futr => "futr",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Tense::Pres => "настоящее время",
            Tense::Past => "прошедшее время",
            Tense::Futr => "будущее время",
        }
    }
}
