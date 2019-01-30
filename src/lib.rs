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
pub mod container;
pub mod dawg;
pub mod estimator;
pub mod opencorpora;
pub mod release;
pub mod shapes;
pub mod util;

pub mod prelude;

pub use crate::{
    analyzer::{units::abc::AnalyzerUnit, MorphAnalyzer},
    container::{
        abc::{MorphySerde, Source},
        Lex, ParseResult, Parsed, Score,
    },
    opencorpora::{Grammeme, GrammemeSet},
};
