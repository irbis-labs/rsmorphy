use std::fmt;

use inlinable_string::InlinableString;

use crate::analyzer::model::abc::*;
use crate::analyzer::model::decode::*;
use crate::analyzer::model::lexeme::Lexeme;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Score;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::tag::OpencorporaTagReg;

#[derive(Debug, Clone, PartialEq)]
pub struct Unknown {
    pub word_lower: InlinableString,
}

impl Unknown {
    #[inline(always)]
    pub fn new<W>(word_lower: W) -> Self
    where
        W: Into<InlinableString>,
    {
        let word_lower = word_lower.into();
        Unknown { word_lower }
    }

    #[inline(always)]
    pub fn word_lower(&self) -> &InlinableString {
        &self.word_lower
    }

    #[inline(always)]
    pub fn into_inner(self) -> InlinableString {
        self.word_lower
    }

    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(&'s self) -> impl Iterator<Item = Lex> + 'i {
        (0..1).map(move |_| Lex::from_stack(StackSource::from(self.clone())))
    }
}

impl Source for Unknown {
    #[inline(always)]
    fn score(&self) -> Score {
        Score::Fake(1.0)
    }

    #[inline(always)]
    fn is_lemma(&self) -> bool {
        // CHECK это ведь то, что мы не смогли выяснить, как оно склоняется?
        // * мы знаем всё о склонении - если не выяснили, значит не склоняется.
        true
    }

    #[inline(always)]
    fn is_known(&self) -> bool {
        false
    }

    #[inline(always)]
    fn get_tag<'a>(&self, morph: &'a MorphAnalyzer) -> &'a OpencorporaTagReg {
        &morph.units().unknown.tag
    }

    #[inline(always)]
    fn try_get_para_id(&self) -> Option<ParadigmId> {
        None
    }

    #[inline(always)]
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(&self.word_lower)
    }

    #[inline(always)]
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, _morph: &MorphAnalyzer) -> fmt::Result {
        f.write_str(&self.word_lower)
    }

    #[inline(always)]
    fn get_lexeme(&self, _morph: &MorphAnalyzer) -> Lexeme {
        self.iter_lexeme().collect()
    }

    #[inline(always)]
    fn get_lemma(&self, _morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme().next().unwrap()
    }
}

impl MorphySerde for Unknown {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str("u:")?;
        for ch in escape(&self.word_lower) {
            f.write_str(ch)?;
        }
        Ok(())
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let s = follow_str(s, "u").map_err(|_| DecodeError::UnknownPartType)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;
        let word_lower = unescape(word).collect();
        Ok((s, Unknown { word_lower }))
    }
}
