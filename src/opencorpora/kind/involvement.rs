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
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "incl" => Some(Involvement::Incl),
                "excl" => Some(Involvement::Excl),
                _ => None,
            })
    }
}
