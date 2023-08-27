use std::fmt;

use crate::analyzer::model::abc::*;
use crate::analyzer::model::decode::*;
use crate::analyzer::model::lexeme::Lexeme;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Score;
use crate::analyzer::model::WordStruct;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::tag::TagReg;

#[derive(Debug, Clone, PartialEq)]
pub struct HyphenAdverb {
    pub(in crate::analyzer) word_lower: WordStruct,
}

impl HyphenAdverb {
    #[inline(always)]
    pub fn new(word_lower: WordStruct) -> Self {
        HyphenAdverb { word_lower }
    }

    #[inline(always)]
    pub fn word_lower(&self) -> &WordStruct {
        &self.word_lower
    }

    #[inline(always)]
    pub fn into_inner(self) -> WordStruct {
        self.word_lower
    }
}

impl Source for HyphenAdverb {
    #[inline(always)]
    fn score(&self) -> Score {
        Score::Fake(0.7)
    }

    #[inline(always)]
    fn is_lemma(&self) -> bool {
        true
    }

    #[inline(always)]
    fn is_known(&self) -> bool {
        self.word_lower.is_known()
    }

    #[inline(always)]
    fn get_tag<'a>(&self, morph: &'a MorphAnalyzer) -> &'a TagReg {
        &morph.units().ha.tag
    }

    #[inline(always)]
    fn try_get_para_id(&self) -> Option<ParadigmId> {
        None
    }

    #[inline(always)]
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(self.word_lower.word())
    }

    #[inline(always)]
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, _morph: &MorphAnalyzer) -> fmt::Result {
        f.write_str(self.word_lower.word())
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

impl HyphenAdverb {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(&'s self) -> impl Iterator<Item = Lex> + 'i {
        (0..1).map(move |_| Lex::from_stack(StackSource::from(self.clone())))
    }
}

impl MorphySerde for HyphenAdverb {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        let word = self.word_lower.word();
        let kind = if self.word_lower.is_known() { "d" } else { "f" };
        write!(f, "ha:{word},{kind}")?;
        Ok(())
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let s = follow_str(s, "ha").map_err(|_| DecodeError::UnknownPartType)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ',')?;
        let (s, is_known) = follow_str(s, "d")
            .map(|s| (s, true))
            .or_else(|_| follow_str(s, "f").map(|s| (s, false)))
            .map_err(|_| DecodeError::UnknownPartType)?;
        Ok((s, HyphenAdverb::new(WordStruct::new(word, is_known))))
    }
}
