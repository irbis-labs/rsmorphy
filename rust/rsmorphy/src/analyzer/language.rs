use std::fmt;

use crate::analyzer::common::Lex;
use crate::analyzer::common::ParseResult;
use crate::analyzer::common::Seen;
use crate::analyzer::common::Token;
use crate::opencorpora::tag::TagReg;
use crate::opencorpora::Dictionary;
use crate::token_codec::DecodeError;

pub trait Lang: fmt::Debug + Clone {
    fn dict(&self) -> &Dictionary;

    fn parse(&self, s: &str) -> ParseResult<Self>
    where
        Self: Sized;

    fn make_seen(&self, lex: &Lex) -> Seen<'_> {
        Seen {
            word: lex.make_word_lower(),
            tag: self.get_tag(lex),
            para_id: lex.left.base.paradigm_id(),
        }
    }

    fn get_tag(&self, lex: &Lex) -> &TagReg;

    fn get_lemma(&self, lex: &Lex) -> Lex {
        self.iter_lexeme(lex).next().unwrap()
    }

    fn iter_lexeme<'s: 'i, 'l: 'i, 'i>(
        &'s self,
        lex: &'l Lex,
    ) -> Box<dyn Iterator<Item = Lex> + 'i>;

    fn encode_token<'m, 'l>(&'m self, lex: &'l Lex) -> Token<'m, 'l, Self> {
        Token::new(self, lex)
    }

    fn write_token(&self, writer: &mut dyn fmt::Write, lex: &Lex) -> fmt::Result;

    fn decode_token(&self, s: &str) -> Result<Lex, DecodeError>;
}
