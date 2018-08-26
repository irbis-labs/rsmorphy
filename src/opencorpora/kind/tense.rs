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
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "pres" => Some(Tense::Pres),
                "past" => Some(Tense::Past),
                "futr" => Some(Tense::Futr),
                _ => None,
            })
    }
}
