#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mood {
    /// изъявительное наклонение
    Indc,
    /// повелительное наклонение
    Impr,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         indc
        |impr
    )
"
);

impl Mood {
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::Mood::*;
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "indc" => Some(Indc),
                "impr" => Some(Impr),
                _ => None,
            })
    }

    pub fn title_rus(self) -> &'static str {
        use self::Mood::*;

        match self {
            Indc => "изъявительное наклонение",
            Impr => "повелительное наклонение",
        }
    }
}
