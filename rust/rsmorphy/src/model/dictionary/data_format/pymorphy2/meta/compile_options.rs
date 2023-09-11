use inlinable_string::InlinableString;
use serde::Deserialize;
use serde::Serialize;
use smallvec::SmallVec;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompileOptions {
    pub min_ending_freq: u32,
    pub min_paradigm_popularity: u32,
    pub max_suffix_length: u8,
    pub paradigm_prefixes: SmallVec<[InlinableString; 8]>,
}
