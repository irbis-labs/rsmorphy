mod common;
mod language;
pub mod ru;
pub mod uk;

pub use common::Lex;
pub use common::Parse;
pub use common::ParseBuffers;
pub use common::ParseContext;
pub use common::ParseResult;
pub use common::Score;
pub use common::Token;
pub use language::Lang;
pub use ru::Russian;
pub use uk::Ukrainian;

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
