#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Aspect {
    /// совершенный вид
    Perf,
    /// несовершенный вид
    Impf,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         perf
        |impf
    )
"
);

impl Aspect {
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        TAG_RE.captures_iter(s.as_ref()).next()
            .and_then(|cap| match &cap[1] {
                "perf" => Some(Aspect::Perf),
                "impf" => Some(Aspect::Impf),
                _ => None,
            })
    }
}
