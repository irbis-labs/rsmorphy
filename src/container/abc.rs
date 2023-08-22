use std::fmt;

use inlinable_string::InlinableString;
use inlinable_string::StringExt;

use crate::analyzer::MorphAnalyzer;
pub use crate::container::decode::error::DecodeError;
use crate::container::Lex;
use crate::container::Score;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::tag::OpencorporaTagReg;

pub trait Source {
    // TODO move out `score` from a word container
    fn score(&self) -> Score;
    fn is_lemma(&self) -> bool;
    fn is_known(&self) -> bool;
    fn get_word(&self) -> InlinableString;
    fn get_normal_form(&self, morph: &MorphAnalyzer) -> InlinableString;
    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg;
    fn try_get_para_id(&self) -> Option<ParadigmId>;
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result;
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result;
    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex>;
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
