use crate::opencorpora::gram::GramId;

/// Число
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Number {
    /// единственное число
    Sing,
    /// множественное число
    Plur,
}

impl Number {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Number::*;

        Some(match id.as_str() {
            "sing" => Sing,
            "plur" => Plur,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            Number::Sing => "sing",
            Number::Plur => "plur",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Number::Sing => "единственное число",
            Number::Plur => "множественное число",
        }
    }
}
