#[cfg_attr(feature = "cargo-clippy", allow(enum_variant_names))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Person {
    /// 1 лицо
    Per1,
    /// 2 лицо
    Per2,
    /// 3 лицо
    Per3,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         1per
        |2per
        |3per
    )
"
);

impl Person {
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "1per" => Some(Person::Per1),
                "2per" => Some(Person::Per2),
                "3per" => Some(Person::Per3),
                _ => None,
            })
    }
}
