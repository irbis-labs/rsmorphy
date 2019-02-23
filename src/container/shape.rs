use std::{borrow::Cow, fmt};

use crate::{
    analyzer::MorphAnalyzer,
    container::{abc::*, decode::*, stack::StackSource, Lex, Score},
    opencorpora::{OpencorporaTagReg, dictionary::ParadigmId},
};

const NUMBER_SCORE: Score = Score::Real(1.0);
const DECAYED_SCORE: Score = Score::Fake(0.9);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ShapeKind {
    Number { is_float: bool },
    RomanNumber,
    Latin,
    Punctuation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shaped {
    word: String,
    kind: ShapeKind,
}

impl Shaped {
    pub fn new<S>(word: S, kind: ShapeKind) -> Self
    where
        S: Into<String>,
    {
        let word = word.into();
        Shaped { word, kind }
    }

    pub fn number<S>(word: S, is_float: bool) -> Self
    where
        S: Into<String>,
    {
        Shaped::new(word, ShapeKind::Number { is_float })
    }

    pub fn roman_number<S>(word: S) -> Self
    where
        S: Into<String>,
    {
        Shaped::new(word, ShapeKind::RomanNumber)
    }

    pub fn latin<S>(word: S) -> Self
    where
        S: Into<String>,
    {
        Shaped::new(word, ShapeKind::Latin)
    }

    pub fn punctuation<S>(word: S) -> Self
    where
        S: Into<String>,
    {
        Shaped::new(word, ShapeKind::Punctuation)
    }

    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        (0..1).map(move |_| Lex::from_stack(morph, StackSource::from(self.clone())))
    }
}

impl Source for Shaped {
    fn score(&self) -> Score {
        match self.kind {
            ShapeKind::Number { .. } => NUMBER_SCORE,
            _ => DECAYED_SCORE,
        }
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
        use self::ShapeKind::*;

        match self.kind {
            Latin => &morph.units.latin.tag,
            RomanNumber => &morph.units.roman.tag,
            Punctuation => &morph.units.punct.tag,
            Number { is_float } => match is_float {
                true => &morph.units.number.tag_real,
                false => &morph.units.number.tag_int,
            },
        }
    }

    fn try_get_para_id(&self) -> Option<ParadigmId> {
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

impl MorphySerde for Shaped {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        use self::ShapeKind::*;

        write!(
            f,
            "s:{},",
            match self.kind {
                Latin => "l",
                Number { is_float } => {
                    if is_float {
                        "f"
                    } else {
                        "i"
                    }
                }
                Punctuation => "p",
                RomanNumber => "r",
            },
        )?;
        for ch in escape(&self.word) {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        use self::ShapeKind::*;

        let s = follow_str(s, "s").map_err(|_| DecodeError::UnknownPartType)?;
        let s = follow_str(s, ":")?;
        let (s, kind) = take_1_char(s)?;
        // FIXME skip escaped ";"
        let (s, word) = take_str_until_char_is(follow_str(s, ",")?, ';')?;
        Ok((
            s,
            Shaped {
                kind: match kind {
                    'l' => Latin,
                    'f' => Number { is_float: true },
                    'i' => Number { is_float: false },
                    'p' => Punctuation,
                    'r' => RomanNumber,
                    _ => Err(DecodeError::UnknownPartType)?,
                },
                word: unescape(word).collect(),
            },
        ))
    }
}
