mod common;
mod language;
pub mod ru;
pub mod uk;

pub use language::Lang;
pub use ru::Russian;
pub use uk::Ukrainian;

pub use crate::analyzer::common::Lex;
pub use crate::analyzer::common::Parse;
pub use crate::analyzer::common::ParseResult;
pub use crate::analyzer::common::Score;
pub use crate::analyzer::common::Token;

#[derive(Debug, Clone)]
pub enum Morph {
    Ru(Russian),
    Uk(Ukrainian),
}

#[derive(Debug, Clone)]
pub enum LexInt {
    Ru { morph: Russian, lex: Lex },
    Uk { morph: Ukrainian, lex: Lex },
}

#[derive(Debug, Clone)]
pub enum LexemeInt {
    Ru {
        morph: Russian,
        // XXX: Lexeme<Ru>
        lexeme: Vec<Lex>,
    },
    Uk {
        morph: Ukrainian,
        // XXX: Lexeme<Uk>
        lexeme: Vec<Lex>,
    },
}

#[derive(Debug, Clone)]
pub enum ParseResultInt {
    Ru { result: ParseResult<Russian> },
    Uk { result: ParseResult<Ukrainian> },
}
