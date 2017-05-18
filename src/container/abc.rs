use std::borrow::Cow;
use std::fmt;

use analyzer::MorphAnalyzer;
use container::Lex;
use container::Score;
use opencorpora::tag::OpencorporaTagReg;

pub use container::decode::error::DecodeError;


pub trait Source {
    fn score(&self) -> Score;
    fn is_lemma(&self) -> bool;
    fn is_known(&self) -> bool;
    fn get_word(&self) -> Cow<str>;
    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str>;
    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg;
    fn try_get_para_id(&self) -> Option<u16>;
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result;
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result;
    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex>;
    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex;
}


pub trait MorphySerde: Sized {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result;
    fn decode(s: &str) -> Result<(&str, Self), DecodeError>;

    fn encoded(&self) -> String {
        let mut s = String::new();
        self.encode(&mut s).unwrap();
        s
    }
//
//    fn encoded<'a, MS: MorphySerde>(&'a self) -> Encoded<'a, MS> {
//
//    }
}


//pub struct Encoded<'a, MS: MorphySerde + 'a> {
//    pub ms: &'a MS
//}
//
//
//impl <'a, MS: MorphySerde> fmt::Display for Encoded<'a, MS> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        self.ms.encode(f)
//    }
//}
