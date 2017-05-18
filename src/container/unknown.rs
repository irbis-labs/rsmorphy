use std::borrow::Cow;
use std::fmt;

use ::analyzer::MorphAnalyzer;
use ::container::Lex;
use ::container::Score;
use ::container::stack::StackSource;
use ::container::abc::*;
use ::opencorpora::tag::OpencorporaTagReg;

use container::decode::*;



#[derive(Debug, Clone, PartialEq)]
pub struct Unknown {
    pub word_lower: String,
}


impl Source for Unknown {
    fn score(&self) -> Score {
        Score::Real(1.0)
    }

    fn is_lemma(&self) -> bool {
        // CHECK это ведь то, что мы не смогли выяснить, как оно склоняется?
        // * мы знаем всё о склонении - если не выяснили, значит не склоняется.
        true
    }

    fn is_known(&self) -> bool {
        false
    }

    fn get_word(&self) -> Cow<str> {
        self.word_lower.as_str().into()
    }

    fn get_normal_form(&self, _morph: &MorphAnalyzer) -> Cow<str> {
        self.word_lower.as_str().into()
    }

    fn get_tag<'a>(&self, morph: &'a MorphAnalyzer) -> &'a OpencorporaTagReg {
        &morph.units.unknown.tag
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


impl Unknown {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(&'s self, morph: &'m MorphAnalyzer) -> impl Iterator<Item = Lex> + 'i {
        (0..1).map(move |_| Lex::from_stack(morph, StackSource::from(self.clone())) )
    }
}


impl MorphySerde for Unknown {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "u:")?;
        for ch in escape(&self.word_lower) {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let s = follow_str(s, "u").map_err(|_| DecodeError::UnknownPartType)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;
        Ok( (s, Unknown {
            word_lower: unescape(word).collect(),
        }) )
    }
}
