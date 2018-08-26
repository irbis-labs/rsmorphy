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
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "tran" => Some(Transitivity::Tran),
                "intr" => Some(Transitivity::Intr),
                _ => None,
            })
    }
}
