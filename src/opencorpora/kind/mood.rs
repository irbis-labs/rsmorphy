#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mood {
    /// изъявительное наклонение
    Indc,
    /// повелительное наклонение
    Impr,
}


regex!(TAG_RE, r"(?x)
    (
         indc
        |impr
    )
");


impl Mood {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "indc"  => Some(Mood::Indc),
                "impr"  => Some(Mood::Impr),
                _       => None,
            },
            None => None,
        }
    }
}
