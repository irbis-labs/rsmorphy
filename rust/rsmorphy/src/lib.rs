#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::match_bool)]
#![allow(clippy::module_inception)]

#[cfg(not(any(feature = "lang-ru", feature = "lang-uk")))]
compile_error!(
    "At least one language must be enabled. Use `--features lang-ru` or `--features lang-uk`"
);

#[cfg(not(any(feature = "dict_load-pymorphy2", feature = "dict_load-include_static")))]
compile_error!(
    "At least one load method must be enabled. \
    Use `--features dict_load-pymorphy2` or `--features dict_load-include_static`"
);

#[cfg(not(any(feature = "dict_repr-classic", feature = "dict_repr-repacked")))]
compile_error!(
    "Dictionary representation must be enabled. \
    Use `--features dict_repr-classic` or `--features dict_repr-repacked`"
);

pub mod analyzer;
pub mod shapes;
pub mod token_codec;
pub mod util;

pub mod prelude;

pub use crate::analyzer::Lex;
pub use crate::analyzer::Parse;
pub use crate::analyzer::ParseResult;
#[cfg(feature = "lang-ru")]
pub use crate::analyzer::Russian;
pub use crate::analyzer::Score;
#[cfg(feature = "lang-uk")]
pub use crate::analyzer::Ukrainian;
