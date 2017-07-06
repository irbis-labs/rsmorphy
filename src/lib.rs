#![feature(conservative_impl_trait)]
//#![feature(field_init_shorthand)]
//#![feature(specialization)]
//#![cfg_attr(test, feature(test))]


#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![cfg_attr(feature = "clippy", allow(items_after_statements))]
#![cfg_attr(feature = "clippy", allow(match_bool))]
#![cfg_attr(feature = "clippy", allow(module_inception))]
// FIXME remove closures with `do catch`
#![cfg_attr(feature = "clippy", allow(redundant_closure_call))]
// TODO post a bug report; https://github.com/Manishearth/rust-clippy/wiki#regex_macro
// false positive, this regex macro implemented in this crate and is not available in the upstream anymore
#![cfg_attr(feature = "clippy", allow(regex_macro))]

#![cfg_attr(feature = "quiet", allow(warnings))]


extern crate base64;
extern crate flate2;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate maplit;
extern crate num;
extern crate regex;
extern crate roman;
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
    MorphAnalyzer::from_file(rsmorphy_dict_ru::DICT_PATH)
}
