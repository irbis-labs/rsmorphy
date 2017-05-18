#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Aspect {
    /// совершенный вид
    Perf,
    /// несовершенный вид
    Impf,
}


regex!(TAG_RE, r"(?x)
    (
         perf
        |impf
    )
");


impl Aspect {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "perf"  => Some(Aspect::Perf),
                "impf"  => Some(Aspect::Impf),
                _       => None,
            },
            None => None,
        }
    }
}
