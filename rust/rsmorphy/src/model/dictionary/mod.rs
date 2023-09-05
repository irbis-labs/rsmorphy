pub mod data_format;
mod dictionary;
mod dictionary_inner;
mod numeral_agreement;
mod paradigm;
mod value;

pub use self::data_format::LoadError;
pub use self::dictionary::*;
pub use self::paradigm::*;
pub use self::value::*;
