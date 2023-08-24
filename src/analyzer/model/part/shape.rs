use std::fmt;

use inlinable_string::InlinableString;

use self::ShapeKind as SK;
use crate::analyzer::model::abc::*;
use crate::analyzer::model::decode::*;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Score;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::OpencorporaTagReg;

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
    pub(in crate::analyzer) word: InlinableString,
    pub(in crate::analyzer) kind: ShapeKind,
}

impl Shaped {
    #[inline(always)]
    pub fn new<S>(word: S, kind: ShapeKind) -> Self
    where
        S: Into<InlinableString>,
    {
        let word = word.into();
        Shaped { word, kind }
    }

    #[inline(always)]
    pub fn number<S>(word: S, is_float: bool) -> Self
    where
        S: Into<InlinableString>,
    {
        Shaped::new(word, ShapeKind::Number { is_float })
    }

    #[inline(always)]
    pub fn roman_number<S>(word: S) -> Self
    where
        S: Into<InlinableString>,
    {
        Shaped::new(word, ShapeKind::RomanNumber)
    }

    #[inline(always)]
    pub fn latin<S>(word: S) -> Self
    where
        S: Into<InlinableString>,
    {
        Shaped::new(word, ShapeKind::Latin)
    }

    #[inline(always)]
    pub fn punctuation<S>(word: S) -> Self
    where
        S: Into<InlinableString>,
    {
        Shaped::new(word, ShapeKind::Punctuation)
    }

    #[inline(always)]
    pub fn word(&self) -> &str {
        &self.word
    }

    #[inline(always)]
    pub fn kind(&self) -> ShapeKind {
        self.kind
    }

    #[inline(always)]
    pub fn into_parts(self) -> (InlinableString, ShapeKind) {
        (self.word, self.kind)
    }

    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
    ) -> impl Iterator<Item = Lex> + 'i {
        (0..1).map(move |_| Lex::from_stack(StackSource::from(self.clone())))
    }
}

impl Source for Shaped {
    #[inline(always)]
    fn score(&self) -> Score {
        match self.kind {
            ShapeKind::Number { .. } => NUMBER_SCORE,
            _ => DECAYED_SCORE,
        }
    }

    #[inline(always)]
    fn is_lemma(&self) -> bool {
        true
    }

    #[inline(always)]
    fn is_known(&self) -> bool {
        false
    }

    #[inline(always)]
    fn get_tag<'a>(&self, morph: &'a MorphAnalyzer) -> &'a OpencorporaTagReg {
        let units = morph.units();
        match self.kind {
            SK::Latin => &units.latin.tag,
            SK::RomanNumber => &units.roman.tag,
            SK::Punctuation => &units.punct.tag,
            SK::Number { is_float: true } => &units.number.tag_real,
            SK::Number { is_float: false } => &units.number.tag_int,
        }
    }

    #[inline(always)]
    fn try_get_para_id(&self) -> Option<ParadigmId> {
        None
    }

    #[inline(always)]
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(&self.word)
    }

    #[inline(always)]
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, _morph: &MorphAnalyzer) -> fmt::Result {
        f.write_str(&self.word)
    }

    #[inline(always)]
    fn get_lexeme(&self, _morph: &MorphAnalyzer) -> Vec<Lex> {
        self.iter_lexeme().collect()
    }

    #[inline(always)]
    fn get_lemma(&self, _morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme().next().unwrap()
    }
}

impl MorphySerde for Shaped {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        let kind = match self.kind {
            SK::Latin => "l",
            SK::Number { is_float: true } => "f",
            SK::Number { is_float: false } => "i",
            SK::Punctuation => "p",
            SK::RomanNumber => "r",
        };
        write!(f, "s:{kind},")?;
        for ch in escape(&self.word) {
            f.write_str(ch)?;
        }
        Ok(())
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let s = follow_str(s, "s").map_err(|_| DecodeError::UnknownPartType)?;
        let s = follow_str(s, ":")?;
        let (s, kind) = take_1_char(s)?;
        // FIXME skip escaped ";"
        let (s, word) = take_str_until_char_is(follow_str(s, ",")?, ';')?;
        let kind = match kind {
            'l' => SK::Latin,
            'f' => SK::Number { is_float: true },
            'i' => SK::Number { is_float: false },
            'p' => SK::Punctuation,
            'r' => SK::RomanNumber,
            _ => Err(DecodeError::UnknownPartType)?,
        };
        let word = unescape(word).collect();
        Ok((s, Shaped { kind, word }))
    }
}
