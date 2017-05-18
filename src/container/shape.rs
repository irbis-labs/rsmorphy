use std::borrow::Cow;
use std::fmt;

use container::Lex;
use container::Score;
use container::abc::*;
use container::stack::StackSource;
use analyzer::MorphAnalyzer;
use opencorpora::OpencorporaTagReg;

use container::decode::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShapeKind {
    Number { is_float: bool },
    RomanNumber,
    Latin,
    Punctuation,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Shaped {
    pub word: String,
    pub kind: ShapeKind,
}


impl Source for Shaped {
    fn score(&self) -> Score {
        Score::Fake(0.9)
    }

    fn is_lemma(&self) -> bool {
        true
    }

    fn is_known(&self) -> bool {
        false
    }

    fn get_word(&self) -> Cow<str> {
        Cow::from(self.word.as_str())
    }

    fn get_normal_form(&self, _morph: &MorphAnalyzer) -> Cow<str> {
        Cow::from(self.word.as_str())
    }

    fn get_tag<'a>(&self, morph: &'a MorphAnalyzer) -> &'a OpencorporaTagReg {
        match self.kind {
            ShapeKind::Latin => &morph.units.latin.tag,
            ShapeKind::RomanNumber => &morph.units.roman.tag,
            ShapeKind::Punctuation => &morph.units.punct.tag,
            ShapeKind::Number { is_float } => match is_float {
                true => &morph.units.number.tag_real,
                false => &morph.units.number.tag_int,
            }
        }
    }

    fn try_get_para_id(&self) -> Option<u16> {
        None
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "{}", self.word)
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, _morph: &MorphAnalyzer) -> fmt::Result {
        write!(f, "{}", self.word)
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        self.iter_lexeme(morph).collect()
    }

    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme(morph).next().unwrap()
    }
}


impl Shaped {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(&'s self, morph: &'m MorphAnalyzer) -> impl Iterator<Item = Lex> + 'i {
        (0..1).map(move |_| Lex::from_stack(morph, StackSource::from(self.clone())) )
    }
}


impl MorphySerde for Shaped {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(
            f, "s:{},",
            match self.kind {
                ShapeKind::Latin => "l",
                ShapeKind::Number { is_float } => if is_float { "f" } else { "i" },
                ShapeKind::Punctuation => "p",
                ShapeKind::RomanNumber => "r",
            },
        )?;
        for ch in escape(&self.word) {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let s = follow_str(s, "s").map_err(|_| DecodeError::UnknownPartType)?;
        let s = follow_str(s, ":")?;
        let (s, kind) = take_1_char(s)?;
        // FIXME skip escaped ";"
        let (s, word) = take_str_until_char_is(follow_str(s, ",")?, ';')?;
        Ok( (s, Shaped {
            kind: match kind {
                'l' => ShapeKind::Latin,
                'f' => ShapeKind::Number{ is_float: true },
                'i' => ShapeKind::Number{ is_float: false },
                'p' => ShapeKind::Punctuation,
                'r' => ShapeKind::RomanNumber,
                _ => unreachable!(),
            },
            word: unescape(word).collect(),
        }) )
    }
}
