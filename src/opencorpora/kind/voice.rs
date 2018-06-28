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
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        match TAG_RE.captures_iter(s.as_ref()).next() {
            Some(ref cap) => match &cap[1] {
                "actv" => Some(Voice::Actv),
                "pssv" => Some(Voice::Pssv),
                _ => None,
            },
            None => None,
        }
    }
}
