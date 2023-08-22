use serde::Deserialize;
use serde::Serialize;
use string_cache::DefaultAtom;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grammeme {
    atom: DefaultAtom,
}

impl Grammeme {
    pub fn new<A: Into<DefaultAtom>>(a: A) -> Self {
        Grammeme { atom: a.into() }
    }
}
