/// Залог
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Voice {
    /// действительный залог
    Actv,
    /// страдательный залог
    Pssv,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         actv
        |pssv
    )
"
);

impl Voice {
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::Voice::*;
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "actv" => Some(Actv),
                "pssv" => Some(Pssv),
                _ => None,
            })
    }

    pub fn title_rus(self) -> &'static str {
        use self::Voice::*;

        match self {
            Actv => "действительный залог",
            Pssv => "страдательный залог",
        }
    }
}
