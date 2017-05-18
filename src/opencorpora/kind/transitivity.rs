#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Transitivity {
    /// переходный
    Tran,
    /// непереходный
    Intr,
}


regex!(TAG_RE, r"(?x)
    (
         tran
        |intr
    )
");


impl Transitivity {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "tran"  => Some(Transitivity::Tran),
                "intr"  => Some(Transitivity::Intr),
                _       => None,
            },
            None => None,
        }
    }
}
