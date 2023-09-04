use inlinable_string::InlinableString;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GramtabFormats {
    #[serde(rename = "opencorpora-int")]
    pub opencorpora_int: InlinableString,
    #[serde(rename = "opencorpora-ext")]
    pub opencorpora_ext: InlinableString,
}
