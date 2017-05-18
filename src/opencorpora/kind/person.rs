#[cfg_attr(feature = "clippy", allow(enum_variant_names))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Person {
    /// 1 лицо
    Per1,
    /// 2 лицо
    Per2,
    /// 3 лицо
    Per3,
}


regex!(TAG_RE, r"(?x)
    (
         1per
        |2per
        |3per
    )
");


impl Person {
    pub fn try_from_str<'a, P>(s: P) -> Option<Self> where P: Into<&'a str> {
        let s = s.into();
        match TAG_RE.captures_iter(s).next() {
            Some(ref cap) => match &cap[1] {
                "1per"  => Some(Person::Per1),
                "2per"  => Some(Person::Per2),
                "3per"  => Some(Person::Per3),
                _       => None,
            },
            None => None,
        }
    }
}
