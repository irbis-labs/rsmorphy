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
        use self::Aspect::*;

        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "perf" => Some(Perf),
                "impf" => Some(Impf),
                _ => None,
            })
    }

    pub fn title_rus(self) -> &'static str {
        use self::Aspect::*;

        match self {
            Perf => "совершенный вид",
            Impf => "несовершенный вид",
        }
    }
}
