use std::borrow::Cow;
use std::fmt;

use ::analyzer::MorphAnalyzer;
use ::container::Lex;
use ::container::Score;
use ::container::stack::StackSource;
//use ::container::Word;
use ::container::abc::*;
use ::opencorpora::tag::OpencorporaTagReg;

use container::decode::*;


#[derive(Debug, Clone, PartialEq)]
pub struct HyphenAdverb {
    pub word_lower: String,
}


impl Source for HyphenAdverb {
    fn score(&self) -> Score {
        Score::Real(0.7)
    }

    fn is_lemma(&self) -> bool {
        true
    }

    fn is_known(&self) -> bool {
        unimplemented!()
//        self.word_lower.is_known
    }

    fn get_word(&self) -> Cow<str> {
        Cow::from(self.word_lower.as_ref())
    }

    fn get_normal_form(&self, _morph: &MorphAnalyzer) -> Cow<str> {
        Cow::from(self.word_lower.as_ref())
    }

    fn get_tag<'a>(&self, morph: &'a MorphAnalyzer) -> &'a OpencorporaTagReg {
        &morph.units.ha.tag
    }

    fn try_get_para_id(&self) -> Option<u16> {
        None
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "{}", self.word_lower)
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, _morph: &MorphAnalyzer) -> fmt::Result {
        write!(f, "{}", self.word_lower)
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        self.iter_lexeme(morph).collect()
    }

    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme(morph).next().unwrap()
    }
}


impl HyphenAdverb {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(&'s self, morph: &'m MorphAnalyzer) -> impl Iterator<Item = Lex> + 'i {
        (0..1).map(move |_| Lex::from_stack(morph, StackSource::from(self.clone())) )
    }
}


impl MorphySerde for HyphenAdverb {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "ha:{}", self.word_lower)
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let s = follow_str(s, "ha").map_err(|_| DecodeError::UnknownPartType)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;
        Ok( (s, HyphenAdverb {
            word_lower: word.to_string(),
        }) )
    }
}
