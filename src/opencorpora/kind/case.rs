use crate::opencorpora::Grammeme;

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

regex!(
    TAG_RE,
    r"(?x)
    (
         nomn
        |gent
        |datv
        |accs
        |ablt
        |loct
        |voct
        |gen1
        |gen2
        |acc2
        |loc1
        |loc2
    )
"
);

impl Case {
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::Case::*;
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "nomn" => Some(Nomn),
                "gent" => Some(Gent),
                "datv" => Some(Datv),
                "accs" => Some(Accs),
                "ablt" => Some(Ablt),
                "loct" => Some(Loct),
                "voct" => Some(Voct),
                "gen1" => Some(Gen1),
                "gen2" => Some(Gen2),
                "acc2" => Some(Acc2),
                "loc1" => Some(Loc1),
                "loc2" => Some(Loc2),
                _ => None,
            })
    }

    pub fn to_grammeme(self) -> Grammeme {
        use self::Case::*;

        match self {
            Nomn => Grammeme::new("nomn"),
            Gent => Grammeme::new("gent"),
            Datv => Grammeme::new("datv"),
            Accs => Grammeme::new("accs"),
            Ablt => Grammeme::new("ablt"),
            Loct => Grammeme::new("loct"),
            Voct => Grammeme::new("voct"),
            Gen1 => Grammeme::new("gen1"),
            Gen2 => Grammeme::new("gen2"),
            Acc2 => Grammeme::new("acc2"),
            Loc1 => Grammeme::new("loc1"),
            Loc2 => Grammeme::new("loc2"),
        }
    }

    pub fn title_rus(self) -> &'static str {
        use self::Case::*;

        match self {
            Nomn => "именительный падеж",
            Gent => "родительный падеж",
            Datv => "дательный падеж",
            Accs => "винительный падеж",
            Ablt => "творительный падеж",
            Loct => "предложный падеж",
            Voct => "звательный падеж",
            Gen1 => "первый родительный падеж",
            Gen2 => "второй родительный (частичный) падеж",
            Acc2 => "второй винительный падеж",
            Loc1 => "первый предложный падеж",
            Loc2 => "второй предложный (местный) падеж",
        }
    }
}
