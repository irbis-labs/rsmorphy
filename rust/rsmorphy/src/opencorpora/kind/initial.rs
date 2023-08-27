use crate::opencorpora::gram::GramId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InitialKind {
    /// Имя.
    FirstName,
    /// Отчество.
    Patronym,
}

impl InitialKind {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::InitialKind::*;

        Some(match id.as_str() {
            "Name" => FirstName,
            "Patr" => Patronym,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            InitialKind::FirstName => "Name",
            InitialKind::Patronym => "Patr",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            InitialKind::FirstName => "имя",
            InitialKind::Patronym => "отчество",
        }
    }
}
