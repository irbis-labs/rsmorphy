#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Involvement {
    /// говорящий включён в действие
    Incl,
    /// говорящий не включён в действие
    Excl,
}


regex!(TAG_RE, r"(?x)
    (
         incl
        |excl
    )
");


impl Involvement {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "incl"  => Some(Involvement::Incl),
                "excl"  => Some(Involvement::Excl),
                _       => None,
            },
            None => None,
        }
    }
}
