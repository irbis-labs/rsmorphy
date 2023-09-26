use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GramtabFormats {
    #[serde(rename = "opencorpora-int")]
    pub opencorpora_int: SmartString,
    #[serde(rename = "opencorpora-ext")]
    pub opencorpora_ext: SmartString,
}
