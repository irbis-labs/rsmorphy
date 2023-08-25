/// Род
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gender {
    /// мужской род
    Masc,
    /// женский род
    Femn,
    /// средний род
    Neut,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         masc
        |femn
        |neut
    )
"
);

impl Gender {
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::Gender::*;
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "masc" => Some(Masc),
                "femn" => Some(Femn),
                "neut" => Some(Neut),
                _ => None,
            })
    }

    pub fn title_rus(self) -> &'static str {
        use self::Gender::*;

        match self {
            Masc => "мужской род",
            Femn => "женский род",
            Neut => "средний род",
        }
    }
}
