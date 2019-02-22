use crate::opencorpora::Grammeme;

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
    pub fn try_from_fmt_int<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        use self::Number::*;
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "sing" => Some(Sing),
                "plur" => Some(Plur),
                _ => None,
            })
    }

    pub fn to_grammeme(self) -> Grammeme {
        use self::Number::*;
        match self {
            Sing => Grammeme::new("sing"),
            Plur => Grammeme::new("plur"),
        }
    }

    pub fn title_rus(self) -> &'static str {
        use self::Number::*;

        match self {
            Sing => "единственное число",
            Plur => "множественное число",
        }
    }
}
