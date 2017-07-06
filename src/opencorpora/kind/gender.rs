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
    pub fn try_from_str<S>(s: S) -> Option<Self> where S: AsRef<str> {
        match TAG_RE.captures_iter(s.as_ref()).next() {
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
