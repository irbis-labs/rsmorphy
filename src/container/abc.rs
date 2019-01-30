use std::{borrow::Cow, fmt};

pub use crate::container::decode::error::DecodeError;

use crate::{
    analyzer::MorphAnalyzer,
    container::{paradigm::ParadigmId, Lex, Score},
    opencorpora::tag::OpencorporaTagReg,
};

pub trait Source {
    // TODO move out `score` from a word container
    fn score(&self) -> Score;
    fn is_lemma(&self) -> bool;
    fn is_known(&self) -> bool;
    fn get_word(&self) -> Cow<str>;
    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str>;
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

    fn encoded(&self) -> String {
        let mut s = String::new();
        self.encode(&mut s).unwrap();
        s
    }
}
