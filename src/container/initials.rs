use std::borrow::Cow;
use std::fmt;

use analyzer::MorphAnalyzer;
use container::abc::*;
use container::decode::*;
use container::paradigm::ParadigmId;
use container::stack::StackSource;
use container::{Lex, Score};
use opencorpora::tag::OpencorporaTagReg;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InitialsKind {
    FirstName,
    Patronym,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Initials {
    pub letter: Cow<'static, str>,
    pub kind: InitialsKind,
    pub tag_idx: u8,
}

impl Initials {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        let base: u8 = match self.kind {
            InitialsKind::FirstName => 0,
            InitialsKind::Patronym => 12,
        };
        (0..morph.units.initials.tags.len() / 2).map(move |tag_idx| {
            let container = Initials {
                tag_idx: base + tag_idx as u8,
                ..self.clone()
            };
            Lex::from_stack(morph, StackSource::from(container))
        })
    }
}

impl Source for Initials {
    fn score(&self) -> Score {
        Score::Real(0.1)
    }

    fn is_lemma(&self) -> bool {
        unimplemented!()
    }

    fn is_known(&self) -> bool {
        unimplemented!()
    }

    fn get_word(&self) -> Cow<str> {
        self.letter.clone()
    }

    fn get_normal_form(&self, _morph: &MorphAnalyzer) -> Cow<str> {
        self.letter.clone()
    }

    fn get_tag<'a>(&self, morph: &'a MorphAnalyzer) -> &'a OpencorporaTagReg {
        &morph.units.initials.tags[self.tag_idx as usize].0
    }

    fn try_get_para_id(&self) -> Option<ParadigmId> {
        None
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "{}", self.letter)
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, _morph: &MorphAnalyzer) -> fmt::Result {
        write!(f, "{}", self.letter)
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        self.iter_lexeme(morph).collect()
    }

    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        self.iter_lexeme(morph).next().unwrap()
    }
}

impl MorphySerde for Initials {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(
            f,
            "i:{}{}{},{}",
            match self.kind {
                InitialsKind::FirstName => "n",
                InitialsKind::Patronym => "p",
            },
            match (self.tag_idx / 6) % 2 {
                0 => "m",
                1 => "f",
                _ => unreachable!(),
            },
            self.tag_idx % 6,
            self.letter
        )
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let s = follow_str(s, "i").map_err(|_| DecodeError::UnknownPartType)?;
        let s = follow_str(s, ":")?;
        let (s, kind) = take_1_char(s)?;
        let (s, gender) = take_1_char(s)?;
        let (s, case) = take_1_char(s)?;
        let (s, word) = take_str_until_char_is(follow_str(s, ",")?, ';')?;
        let letter = Cow::from(word.to_string());
        let tag_idx = decode_tag_idx(kind, gender, case)?;
        let kind = match kind {
            'n' => InitialsKind::FirstName,
            'p' => InitialsKind::Patronym,
            // FIXME is it proper error?
            _ => Err(DecodeError::UnknownPartType)?,
        };
        Ok((
            s,
            Initials {
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
