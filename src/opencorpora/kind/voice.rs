/// Залог
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Voice {
    /// действительный залог
    Actv,
    /// страдательный залог
    Pssv,
}


regex!(TAG_RE, r"(?x)
    (
         actv
        |pssv
    )
");


impl Voice {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "actv"  => Some(Voice::Actv),
                "pssv"  => Some(Voice::Pssv),
                _       => None,
            },
            None => None,
        }
    }
}
