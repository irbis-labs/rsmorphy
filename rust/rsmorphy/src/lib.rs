#![deny(bare_trait_objects)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::match_bool)]
#![allow(clippy::module_inception)]

pub mod analyzer;
pub mod dawg;
pub mod opencorpora;
pub mod shapes;
pub mod token_codec;
pub mod util;

pub mod prelude;

pub use crate::analyzer::Lex;
pub use crate::analyzer::Parse;
pub use crate::analyzer::ParseResult;
pub use crate::analyzer::Russian;
pub use crate::analyzer::Score;
pub use crate::analyzer::Ukrainian;
pub use crate::opencorpora::Gram;
pub use crate::opencorpora::GramReg;
pub use crate::opencorpora::GramSet;
