use std::fmt;

use crate::analyzer::model::abc::*;
use crate::analyzer::model::decode::*;
use crate::analyzer::model::lexeme::Lexeme;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Score;
use crate::analyzer::units::abbr::initial::LETTERS;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::kind::InitialKind;
use crate::opencorpora::tag::TagReg;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Initial {
    pub(in crate::analyzer) letter: &'static str,
    pub(in crate::analyzer) kind: InitialKind,
    pub(in crate::analyzer) tag_idx: u8,
}

impl Initial {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        let base: u8 = match self.kind {
            InitialKind::FirstName => 0,
            InitialKind::Patronym => 12,
        };
        (0..morph.units().initial.tags.len() / 2).map(move |tag_idx| {
            let container = Initial {
                tag_idx: base + tag_idx as u8,
                ..*self
            };
            Lex::from_stack(StackSource::from(container))
        })
    }

    #[inline(always)]
    pub fn letter(&self) -> &'static str {
        self.letter
    }

    #[inline(always)]
    pub fn kind(&self) -> InitialKind {
        self.kind
    }
}

impl Source for Initial {
    #[inline(always)]
    fn score(&self) -> Score {
        Score::Real(0.1)
    }

    #[inline(always)]
    fn is_lemma(&self) -> bool {
        unimplemented!()
    }

    #[inline(always)]
    fn is_known(&self) -> bool {
        unimplemented!()
    }

    #[inline(always)]
    fn get_tag<'a>(&self, morph: &'a MorphAnalyzer) -> &'a TagReg {
        &morph.units().initial.tags[self.tag_idx as usize].0
    }

    #[inline(always)]
    fn try_get_para_id(&self) -> Option<ParadigmId> {
        None
    }

    #[inline(always)]
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(&self.letter)
    }

    #[inline(always)]
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, _morph: &MorphAnalyzer) -> fmt::Result {
        f.write_str(self.letter)
    }

    #[inline(always)]
    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Lexeme {
        self.iter_lexeme(morph).collect()
    }

    #[inline(always)]
    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme(morph).next().unwrap()
    }
}

impl MorphySerde for Initial {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(
            f,
            "i:{kind}{gender}{case},{letter}",
            kind = match self.kind {
                InitialKind::FirstName => "n",
                InitialKind::Patronym => "p",
            },
            gender = match (self.tag_idx / 6) % 2 {
                0 => "m",
                1 => "f",
                _ => unreachable!(),
            },
            case = self.tag_idx % 6,
            letter = self.letter
        )
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let s = follow_str(s, "i").map_err(|_| DecodeError::UnknownPartType)?;
        let s = follow_str(s, ":")?;
        let (s, kind) = take_1_char(s)?;
        let (s, gender) = take_1_char(s)?;
        let (s, case) = take_1_char(s)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ",")?, ';')?;
        let tag_idx = decode_tag_idx(kind, gender, case)?;
        let kind = match kind {
            'n' => InitialKind::FirstName,
            'p' => InitialKind::Patronym,
            // FIXME is it proper error?
            _ => Err(DecodeError::UnknownPartType)?,
        };
        let &letter = LETTERS.get(word).ok_or_else(|| DecodeError::DoesntMatch)?;
        Ok((
            s,
            Initial {
                kind,
                tag_idx,
                letter,
            },
        ))
    }
}

fn decode_tag_idx(kind: char, gender: char, case: char) -> Result<u8, DecodeError> {
    let kind = match kind {
        'n' => 0,
        'p' => 1,
        _ => Err(DecodeError::UnknownPartType)?,
    };
    let gender = match gender {
        'm' => 0,
        'f' => 1,
        _ => Err(DecodeError::UnknownPartType)?,
    };
    let case = match case {
        '0'..='5' => case as u8 - b'0',
        _ => Err(DecodeError::UnknownPartType)?,
    };
    Ok(kind * 12 + gender * 6 + case)
}
