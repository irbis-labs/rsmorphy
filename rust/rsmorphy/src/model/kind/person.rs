use crate::model::gram::GramId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Person {
    /// 1 лицо
    Per1,
    /// 2 лицо
    Per2,
    /// 3 лицо
    Per3,
}

impl Person {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Person::*;

        Some(match id.as_str() {
            "1per" => Per1,
            "2per" => Per2,
            "3per" => Per3,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            Person::Per1 => "1per",
            Person::Per2 => "2per",
            Person::Per3 => "3per",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Person::Per1 => "1 лицо",
            Person::Per2 => "2 лицо",
            Person::Per3 => "3 лицо",
        }
    }
}
