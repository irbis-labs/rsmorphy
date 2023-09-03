use crate::opencorpora::gram::GramId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Involvement {
    /// говорящий включён в действие
    Incl,
    /// говорящий не включён в действие
    Excl,
}

impl Involvement {
    pub fn gram_int(self) -> &'static str {
        match self {
            Involvement::Incl => "incl",
            Involvement::Excl => "excl",
        }
    }

    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Involvement::*;

        Some(match id.as_str() {
            "incl" => Incl,
            "excl" => Excl,
            _ => None?,
        })
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Involvement::Incl => "говорящий включён в действие",
            Involvement::Excl => "говорящий не включён в действие",
        }
    }
}
