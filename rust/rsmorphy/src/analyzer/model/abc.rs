use std::fmt;

use inlinable_string::InlinableString;
use inlinable_string::StringExt;

pub use crate::analyzer::model::decode::error::DecodeError;
use crate::analyzer::model::lexeme::Lexeme;
use crate::analyzer::model::Lex;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::tag::TagReg;

pub trait Source {
    fn is_lemma(&self) -> bool;
    fn is_known(&self) -> bool;
    fn get_word(&self) -> InlinableString {
        let mut s = InlinableString::new();
        self.write_word(&mut s).expect("write_word");
        s
    }
    fn get_normal_form(&self, morph: &MorphAnalyzer) -> InlinableString {
        let mut s = InlinableString::new();
        self.write_normal_form(&mut s, morph)
            .expect("write_normal_form");
        s
    }
    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m TagReg;
    fn try_get_para_id(&self) -> Option<ParadigmId>;
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result;
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result;
    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Lexeme;
    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex;
}

pub trait MorphySerde: Sized {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result;
    fn decode(s: &str) -> Result<(&str, Self), DecodeError>;

    fn encoded(&self) -> InlinableString {
        let mut s = InlinableString::new();
        self.encode(&mut s).expect("encode");
        s
    }
}
