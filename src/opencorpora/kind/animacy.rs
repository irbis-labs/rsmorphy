#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Animacy {
    /// одушевлённое
    Anim,
    /// неодушевлённое
    Inan,
}

regex!(
    TAG_RE,
    r"(?x)
    (
         anim
        |inan
    )
"
);

impl Animacy {
    pub fn try_from_str<S>(s: S) -> Option<Self>
    where
        S: AsRef<str>,
    {
        TAG_RE
            .captures_iter(s.as_ref())
            .next()
            .and_then(|cap| match &cap[1] {
                "anim" => Some(Animacy::Anim),
                "inan" => Some(Animacy::Inan),
                _ => None,
            })
    }
}
