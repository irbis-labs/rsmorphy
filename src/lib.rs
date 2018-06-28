#![deny(bare_trait_objects)]
#![deny(missing_copy_implementations)]

#![cfg_attr(feature = "cargo-clippy", allow(items_after_statements))]
#![cfg_attr(feature = "cargo-clippy", allow(match_bool))]
#![cfg_attr(feature = "cargo-clippy", allow(module_inception))]
// TODO post a bug report; https://github.com/Manishearth/rust-clippy/wiki#regex_macro
// false positive, this regex macro implemented in this crate and is not available in the upstream anymore
#![cfg_attr(feature = "cargo-clippy", allow(regex_macro))]

#![cfg_attr(feature = "quiet", allow(warnings))]


extern crate base64;
extern crate flate2;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate num;
extern crate regex;
extern crate roman;
extern crate serde_json;
extern crate string_cache;
extern crate unicode_categories as uc;

pub extern crate rsmorphy_dict_ru;

#[macro_use]
pub mod macros;

pub mod analyzer;
pub mod container;
pub mod dawg;
pub mod estimator;
pub mod opencorpora;
pub mod release;
pub mod shapes;
pub mod util;

pub mod prelude;


pub use analyzer::units::abc::AnalyzerUnit;
pub use container::abc::Source;
pub use container::abc::MorphySerde;

pub use analyzer::MorphAnalyzer;
pub use container::Lex;
pub use container::Parsed;
pub use container::ParseResult;
pub use container::Score;
pub use opencorpora::Grammeme;
pub use opencorpora::GrammemeSet;
