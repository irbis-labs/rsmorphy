#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Animacy {
    /// одушевлённое
    Anim,
    /// неодушевлённое
    Inan,
}


regex!(TAG_RE, r"(?x)
    (
         anim
        |inan
    )
");


impl Animacy {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "anim"  => Some(Animacy::Anim),
                "inan"  => Some(Animacy::Inan),
                _       => None,
            },
            None => None,
        }
    }
}
