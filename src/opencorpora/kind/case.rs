use opencorpora::Grammeme;

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
        TAG_RE.captures_iter(s.as_ref()).next()
            .and_then(|cap| match &cap[1] {
                "nomn" => Some(Case::Nomn),
                "gent" => Some(Case::Gent),
                "datv" => Some(Case::Datv),
                "accs" => Some(Case::Accs),
                "ablt" => Some(Case::Ablt),
                "loct" => Some(Case::Loct),
                "voct" => Some(Case::Voct),
                "gen1" => Some(Case::Gen1),
                "gen2" => Some(Case::Gen2),
                "acc2" => Some(Case::Acc2),
                "loc1" => Some(Case::Loc1),
                "loc2" => Some(Case::Loc2),
                _ => None,
            })
    }

    pub fn to_grammeme(self) -> Grammeme {
        match self {
            Case::Nomn => Grammeme::new("nomn"),
            Case::Gent => Grammeme::new("gent"),
            Case::Datv => Grammeme::new("datv"),
            Case::Accs => Grammeme::new("accs"),
            Case::Ablt => Grammeme::new("ablt"),
            Case::Loct => Grammeme::new("loct"),
            Case::Voct => Grammeme::new("voct"),
            Case::Gen1 => Grammeme::new("gen1"),
            Case::Gen2 => Grammeme::new("gen2"),
            Case::Acc2 => Grammeme::new("acc2"),
            Case::Loc1 => Grammeme::new("loc1"),
            Case::Loc2 => Grammeme::new("loc2"),
        }
    }
}
