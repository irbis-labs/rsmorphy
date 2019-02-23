use boolinator::Boolinator;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use string_cache::DefaultAtom;

use crate::opencorpora::Grammeme;

#[derive(Deserialize, Serialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrammemeReg {
    pub name: Grammeme,
    pub parent: Option<Grammeme>,
    pub alias: DefaultAtom,
    pub description: DefaultAtom,
}

impl GrammemeReg {
    pub fn from_json(array: Vec<Value>) -> Self {
        assert_eq!(array.len(), 4, "Expected 4 elements in array");
        let mut array = array.into_iter().map(|v| match v {
            Value::Null => None,
            Value::String(string) => (!string.is_empty()).as_some(string),
            wrong_value => panic!(
                "Expected null or non-empty string, found: {:?}",
                wrong_value
            ),
        });
        let mut next = || array.next().unwrap();
        GrammemeReg {
            name: next().map(Grammeme::new).expect("string"),
            parent: next().map(Grammeme::new),
            alias: next().map(From::from).expect("string"),
            description: next().map(From::from).expect("string"),
        }
    }
}
