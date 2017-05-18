use opencorpora::Grammeme;


/// Число
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Number {
    /// единственное число
    Sing,
    /// множественное число
    Plur,
}


regex!(TAG_RE, r"(?x)
    (
         sing
        |plur
    )
");


impl Number {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "sing"  => Some(Number::Sing),
                "plur"  => Some(Number::Plur),
                _       => None,
            },
            None => None,
        }
    }

    pub fn to_grammeme(&self) -> Grammeme {
        match *self {
            Number::Sing => Grammeme::new("sing"),
            Number::Plur => Grammeme::new("plur"),
        }
    }
}
