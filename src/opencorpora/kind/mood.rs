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
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        TAG_RE.captures_iter(s.as_ref()).next()
            .and_then(|cap| match &cap[1] {
                "indc" => Some(Mood::Indc),
                "impr" => Some(Mood::Impr),
                _ => None,
            })
    }
}
