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
    pub fn from_json(data: &Value) -> Self {
        let data = data.as_array().unwrap();
        let get_str = move |i| {
            let v: &Value = &data[i];
            if v.is_null() { return None }
            let v: &str = v.as_str().unwrap();
            if v.is_empty() { return None }
            Some(v.into())
        };
        GrammemeReg {
            name: get_str(0).map(Grammeme::new).unwrap(),
            parent: get_str(1).map(Grammeme::new),
            alias: get_str(2).unwrap(),
            description: get_str(3).unwrap(),
        }
    }

    pub fn map_from_json(data: Value) -> HashMap<Grammeme, Self> {
        let data = data.as_array().unwrap();
        data.into_iter().map(|v| {
            let grammeme = GrammemeReg::from_json(v);
            (grammeme.name.clone(), grammeme)
        })
            .collect()
    }
}
