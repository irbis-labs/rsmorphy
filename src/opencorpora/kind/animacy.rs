#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Animacy {
    /// одушевлённое
    Anim,
    /// неодушевлённое
    Inan,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         anim
        |inan
    )
"
);

impl Animacy {
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::Animacy::*;

        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "anim" => Some(Anim),
                "inan" => Some(Inan),
                _ => None,
            })
    }

    pub fn title_rus(self) -> &'static str {
        use self::Animacy::*;

        match self {
            Anim => "одушевлённое",
            Inan => "неодушевлённое",
        }
    }
}
