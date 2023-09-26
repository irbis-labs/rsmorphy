use serde::Deserialize;
use serde::Serialize;
use smallvec::SmallVec;
use smart_string::SmartString;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompileOptions {
    pub min_ending_freq: u32,
    pub min_paradigm_popularity: u32,
    pub max_suffix_length: u8,
    pub paradigm_prefixes: SmallVec<[SmartString; 8]>,
}
