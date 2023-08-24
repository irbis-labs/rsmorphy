#![deny(bare_trait_objects)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::match_bool)]
#![allow(clippy::module_inception)]
// TODO post a bug report; https://github.com/Manishearth/rust-clippy/wiki#regex_macro
// false positive, this regex macro implemented in this crate and is not available in the upstream anymore
#![allow(clippy::regex_macro)]

#[macro_use]
pub mod macros;

pub mod analyzer;
pub mod dawg;
pub mod estimator;
pub mod opencorpora;
pub mod shapes;
pub mod util;

pub mod prelude;

pub use crate::analyzer::model::abc::MorphySerde;
pub use crate::analyzer::model::abc::Source;
pub use crate::analyzer::model::Lex;
pub use crate::analyzer::model::ParseResult;
pub use crate::analyzer::model::Parsed;
pub use crate::analyzer::model::Score;
pub use crate::analyzer::unit::AnalyzerUnit;
pub use crate::analyzer::MorphAnalyzer;
pub use crate::opencorpora::Grammeme;
pub use crate::opencorpora::GrammemeSet;
