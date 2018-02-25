use std::collections::HashMap;

use serde_json::Value;

use opencorpora::Grammeme;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrammemeReg {
    pub name: Grammeme,
    pub parent: Option<Grammeme>,
    pub alias: String,
    pub description: String,
}

impl GrammemeReg {
    pub fn from_json(json: Value) -> Self {
        match json {
            Value::Array(array) => {
                let get_str = move |i: usize| {
                    let v: &Value = &array[i];
                    if v.is_null() { return None }
                    let v: &str = v.as_str().unwrap();
                    if v.is_empty() { return None }
                    Some(v.into())
                };
                GrammemeReg {
                    name: get_str(0).map(Grammeme::new).expect("Expected string"),
                    parent: get_str(1).map(Grammeme::new),
                    alias: get_str(2).expect("Expected string"),
                    description: get_str(3).expect("Expected string"),
                }
            },
            wrong_value => panic!("Expected array, found: {:?}", wrong_value),
        }
    }

    pub fn map_from_json(data: Value) -> HashMap<Grammeme, Self> {
        match data {
            Value::Array(array) => array.into_iter()
                .map(|v| {
                    let grammeme = GrammemeReg::from_json(v);
                    (grammeme.name.clone(), grammeme)
                })
                .collect(),
            wrong_value => panic!("Expected array, found: {:?}", wrong_value),
        }
    }
}
