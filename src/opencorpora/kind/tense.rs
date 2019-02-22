#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tense {
    /// настоящее время
    Pres,
    /// прошедшее время
    Past,
    /// будущее время
    Futr,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         pres
        |past
        |futr
    )
"
);

impl Tense {
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::Tense::*;
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "pres" => Some(Pres),
                "past" => Some(Past),
                "futr" => Some(Futr),
                _ => None,
            })
    }

    pub fn title_rus(self) -> &'static str {
        use self::Tense::*;

        match self {
            Pres => "настоящее время",
            Past => "прошедшее время",
            Futr => "будущее время",
        }
    }
}
