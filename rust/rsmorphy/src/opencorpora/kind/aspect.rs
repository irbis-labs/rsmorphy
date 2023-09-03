use crate::opencorpora::gram::GramId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Aspect {
    /// совершенный вид
    Perf,
    /// несовершенный вид
    Impf,
}

impl Aspect {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Aspect::*;

        Some(match id.as_str() {
            "perf" => Perf,
            "impf" => Impf,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            Aspect::Perf => "perf",
            Aspect::Impf => "impf",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Aspect::Perf => "совершенный вид",
            Aspect::Impf => "несовершенный вид",
        }
    }
}
