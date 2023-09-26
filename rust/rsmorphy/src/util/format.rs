use std::fmt;

use smart_string::SmartString;

use crate::analyzer::Lang;
use crate::analyzer::Lex;
use crate::analyzer::Parse;
use crate::analyzer::Score;

#[derive(Debug, Clone, Copy)]
pub struct ExplainParse<'s, L: Lang> {
    pub morph: &'s L,
    pub parse: &'s Parse,
}

impl<'s, L: Lang> ExplainParse<'s, L> {
    pub fn new(morph: &'s L, parse: &'s Parse) -> Self {
        ExplainParse { morph, parse }
    }
}

impl<'s, L: Lang> fmt::Display for ExplainParse<'s, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { morph, parse } = *self;
        let (rf, score) = match parse.score {
            Score::Real(score) => ("R", score),
            Score::Fake(score) => ("F", score),
        };
        let lex = ExplainLex::new(morph, &parse.lex);
        write!(f, "{rf} {score:7.5} :: {lex}",)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExplainLex<'s, L: Lang> {
    pub morph: &'s L,
    pub lex: &'s Lex,
}

impl<'s, L: Lang> ExplainLex<'s, L> {
    pub fn new(morph: &'s L, lex: &'s Lex) -> Self {
        ExplainLex { morph, lex }
    }
}

impl<'s, L: Lang> fmt::Display for ExplainLex<'s, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { morph, lex } = *self;

        let word = lex.make_word_lower();
        let norm = lex.make_normal_form(morph.dict());
        let mut enc = SmartString::<254>::new();
        morph.write_token(&mut enc, lex).unwrap();
        let tag = &morph.get_tag(lex).fmt_int;

        write!(f, "{word:20} — {norm:20} :: {enc:35} :: {tag:40}")
    }
}
