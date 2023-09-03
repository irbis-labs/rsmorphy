use crate::opencorpora::gram::GramId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Animacy {
    /// одушевлённое
    Anim,
    /// неодушевлённое
    Inan,
}

impl Animacy {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Animacy::*;

        Some(match id.as_str() {
            "anim" => Anim,
            "inan" => Inan,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            Animacy::Anim => "anim",
            Animacy::Inan => "inan",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Animacy::Anim => "одушевлённое",
            Animacy::Inan => "неодушевлённое",
        }
    }
}
