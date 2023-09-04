use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageCode {
    Ru,
    Uk,
}
