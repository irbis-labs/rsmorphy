#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Transitivity {
    /// переходный
    Tran,
    /// непереходный
    Intr,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         tran
        |intr
    )
"
);

impl Transitivity {
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::Transitivity::*;
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "tran" => Some(Tran),
                "intr" => Some(Intr),
                _ => None,
            })
    }

    pub fn title_rus(self) -> &'static str {
        use self::Transitivity::*;

        match self {
            Tran => "переходный",
            Intr => "непереходный",
        }
    }
}
