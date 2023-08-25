#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Involvement {
    /// говорящий включён в действие
    Incl,
    /// говорящий не включён в действие
    Excl,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         incl
        |excl
    )
"
);

impl Involvement {
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::Involvement::*;
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "incl" => Some(Incl),
                "excl" => Some(Excl),
                _ => None,
            })
    }

    pub fn title_rus(self) -> &'static str {
        use self::Involvement::*;

        match self {
            Incl => "говорящий включён в действие",
            Excl => "говорящий не включён в действие",
        }
    }
}
