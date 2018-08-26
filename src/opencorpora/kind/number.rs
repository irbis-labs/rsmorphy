use opencorpora::Grammeme;

/// Число
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Number {
    /// единственное число
    Sing,
    /// множественное число
    Plur,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         sing
        |plur
    )
"
);

impl Number {
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        TAG_RE.captures_iter(s.as_ref()).next()
            .and_then(|cap| match &cap[1] {
                "sing" => Some(Number::Sing),
                "plur" => Some(Number::Plur),
                _ => None,
            })
    }

    pub fn to_grammeme(self) -> Grammeme {
        match self {
            Number::Sing => Grammeme::new("sing"),
            Number::Plur => Grammeme::new("plur"),
        }
    }
}
