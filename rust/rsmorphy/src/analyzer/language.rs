use std::fmt;

use lib::types::tag::TagReg;

use crate::analyzer::common::Lex;
use crate::analyzer::common::ParseBuffers;
use crate::analyzer::common::ParseContext;
use crate::analyzer::common::ParseResult;
use crate::analyzer::common::Seen;
use crate::analyzer::common::Token;
use crate::analyzer::Dictionary;
use crate::token_codec::DecodeError;

pub trait Lang: fmt::Debug + Clone {
    fn dict(&self) -> &Dictionary;

    fn parse_one(&self, word: &str) -> ParseResult<Self>
    where
        Self: Sized,
    {
        ParseBuffers::new(self)
            .prepare(word)
            .parse()
            .estimate()
            .take_result()
    }

    fn make_buffer(&self) -> ParseBuffers<Self>
    where
        Self: Sized,
    {
        ParseBuffers::new(self)
    }

    fn parse_prepared<'m, 'w, 'b>(&self, context: &mut ParseContext<'m, 'w, 'b, Self>)
    where
        Self: Sized;

    fn estimate_prepared<'m, 'w, 'b>(&self, context: &mut ParseContext<'m, 'w, 'b, Self>)
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
