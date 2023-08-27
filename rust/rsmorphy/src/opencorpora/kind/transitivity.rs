use serde::Deserialize;
use serde::Serialize;

use crate::opencorpora::gram::GramId;

#[derive(Serialize, Deserialize)]
//
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Transitivity {
    /// переходный
    Tran,
    /// непереходный
    Intr,
}

impl Transitivity {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Transitivity::*;

        Some(match id.as_str() {
            "tran" => Tran,
            "intr" => Intr,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            Transitivity::Tran => "tran",
            Transitivity::Intr => "intr",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Transitivity::Tran => "переходный",
            Transitivity::Intr => "непереходный",
        }
    }
}
