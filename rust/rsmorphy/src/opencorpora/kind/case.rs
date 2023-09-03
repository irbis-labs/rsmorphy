use crate::opencorpora::gram::GramId;

/// Падеж
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Case {
    /// именительный падеж
    Nomn,
    /// родительный падеж
    Gent,
    /// дательный падеж
    Datv,
    /// винительный падеж
    Accs,
    /// творительный падеж
    Ablt,
    /// предложный падеж
    Loct,
    /// звательный падеж
    Voct,
    /// первый родительный падеж
    Gen1,
    /// второй родительный (частичный) падеж
    Gen2,
    /// второй винительный падеж
    Acc2,
    /// первый предложный падеж
    Loc1,
    /// второй предложный (местный) падеж
    Loc2,
}

impl Case {
    pub fn from_gram_id(id: GramId) -> Option<Self> {
        use self::Case::*;

        Some(match id.as_str() {
            "nomn" => Nomn,
            "gent" => Gent,
            "datv" => Datv,
            "accs" => Accs,
            "ablt" => Ablt,
            "loct" => Loct,
            "voct" => Voct,
            "gen1" => Gen1,
            "gen2" => Gen2,
            "acc2" => Acc2,
            "loc1" => Loc1,
            "loc2" => Loc2,
            _ => None?,
        })
    }

    pub fn gram_int(self) -> &'static str {
        match self {
            Case::Nomn => "nomn",
            Case::Gent => "gent",
            Case::Datv => "datv",
            Case::Accs => "accs",
            Case::Ablt => "ablt",
            Case::Loct => "loct",
            Case::Voct => "voct",
            Case::Gen1 => "gen1",
            Case::Gen2 => "gen2",
            Case::Acc2 => "acc2",
            Case::Loc1 => "loc1",
            Case::Loc2 => "loc2",
        }
    }

    pub fn title_rus(self) -> &'static str {
        match self {
            Case::Nomn => "именительный падеж",
            Case::Gent => "родительный падеж",
            Case::Datv => "дательный падеж",
            Case::Accs => "винительный падеж",
            Case::Ablt => "творительный падеж",
            Case::Loct => "предложный падеж",
            Case::Voct => "звательный падеж",
            Case::Gen1 => "первый родительный падеж",
            Case::Gen2 => "второй родительный (частичный) падеж",
            Case::Acc2 => "второй винительный падеж",
            Case::Loc1 => "первый предложный падеж",
            Case::Loc2 => "второй предложный (местный) падеж",
        }
    }
}
