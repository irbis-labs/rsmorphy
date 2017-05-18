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


regex!(TAG_RE, r"(?x)
    (
         masc
        |femn
        |neut
    )
");


impl Gender {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "masc"  => Some(Gender::Masc),
                "femn"  => Some(Gender::Femn),
                "neut"  => Some(Gender::Neut),
                _       => None,
            },
            None => None,
        }
    }
}
