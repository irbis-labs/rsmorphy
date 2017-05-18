#![feature(conservative_impl_trait)]
//#![feature(field_init_shorthand)]
//#![feature(specialization)]
//#![cfg_attr(test, feature(test))]


#![cfg_attr(feature = "clippy", allow(items_after_statements))]
#![cfg_attr(feature = "clippy", allow(match_bool))]

#![cfg_attr(feature = "quiet", allow(warnings))]


use std::path::Path;


extern crate flate2;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate maplit;
extern crate num;
extern crate regex;
extern crate roman;
extern crate rustc_serialize;
extern crate serde_json;
extern crate string_cache;
extern crate unicode_categories as uc;

extern crate rsmorphy_dict_ru;


#[macro_use] pub mod macros;

pub mod analyzer;
pub mod container;
pub mod dawg;
pub mod opencorpora;
pub mod release;
pub mod shapes;
pub mod util;


pub mod prelude;


pub use analyzer::units::abc::Analyzer;
pub use container::abc::Source;
pub use container::abc::MorphySerde;

pub use analyzer::MorphAnalyzer;
pub use container::Lex;
pub use container::Parsed;
pub use container::ParseResult;
pub use container::Score;
pub use opencorpora::Grammeme;
pub use opencorpora::GrammemeSet;


pub fn load_test_morph_ru() -> MorphAnalyzer {
    MorphAnalyzer::from_file(&Path::new(rsmorphy_dict_ru::DICT_PATH))
}

