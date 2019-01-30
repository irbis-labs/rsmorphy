use std::{borrow::Cow, fmt};

use crate::{
    analyzer::MorphAnalyzer,
    container::{
        abc::*, paradigm::ParadigmId, Dictionary, HyphenAdverb, Initials, Lex, Score, Shaped,
        Unknown,
    },
    opencorpora::OpencorporaTagReg,
};

use self::StackSource::*;

#[derive(Debug, Clone, PartialEq)]
pub enum StackSource {
    Dictionary(Dictionary),
    HyphenAdverb(HyphenAdverb),
    Initials(Initials),
    Shaped(Shaped),
    Unknown(Unknown),
}

impl StackSource {
    pub fn new<T>(source: T) -> Self
    where
        T: Into<StackSource>,
    {
        source.into()
    }

    pub fn as_dictionary(&self) -> Option<&Dictionary> {
        match self {
            Dictionary(source) => Some(source),
            _ => None,
        }
    }

    pub fn as_hyphen_adverb(&self) -> Option<&HyphenAdverb> {
        match self {
            HyphenAdverb(source) => Some(source),
            _ => None,
        }
    }

    pub fn as_initials(&self) -> Option<&Initials> {
        match self {
            Initials(source) => Some(source),
            _ => None,
        }
    }

    pub fn as_shaped(&self) -> Option<&Shaped> {
        match self {
            Shaped(source) => Some(source),
            _ => None,
        }
    }

    pub fn as_unknown(&self) -> Option<&Unknown> {
        match self {
            Unknown(source) => Some(source),
            _ => None,
        }
    }

    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> Box<dyn Iterator<Item = Lex> + 'i> {
        match self {
            Dictionary(source) => Box::new(source.iter_lexeme(morph)),
            HyphenAdverb(source) => Box::new(source.iter_lexeme(morph)),
            Initials(source) => Box::new(source.iter_lexeme(morph)),
            Shaped(source) => Box::new(source.iter_lexeme(morph)),
            Unknown(source) => Box::new(source.iter_lexeme(morph)),
        }
    }

    pub fn title_rus(&self) -> &'static str {
        match self {
            Dictionary(dict_source) => match dict_source.word_lower().is_known() {
                true => "Словарное слово",
                false => "Неизвестное слово",
            },
            HyphenAdverb(_) => "Наречие с дефисом",
            Initials(_) => "Инициал",
            Shaped(_) => "Не слово",
            Unknown(_) => "Неизвестное слово",
        }
    }
}

impl From<Dictionary> for StackSource {
    fn from(source: Dictionary) -> Self {
        Dictionary(source)
    }
}

impl From<HyphenAdverb> for StackSource {
    fn from(source: HyphenAdverb) -> Self {
        HyphenAdverb(source)
    }
}

impl From<Initials> for StackSource {
    fn from(source: Initials) -> Self {
        Initials(source)
    }
}

impl From<Shaped> for StackSource {
    fn from(source: Shaped) -> Self {
        Shaped(source)
    }
}

impl From<Unknown> for StackSource {
    fn from(source: Unknown) -> Self {
        Unknown(source)
    }
}

impl Source for StackSource {
    fn score(&self) -> Score {
        match *self {
            Dictionary(ref source) => source.score(),
            HyphenAdverb(ref source) => source.score(),
            Initials(ref source) => source.score(),
            Shaped(ref source) => source.score(),
            Unknown(ref source) => source.score(),
        }
    }

    fn is_lemma(&self) -> bool {
        match *self {
            Dictionary(ref source) => source.is_lemma(),
            HyphenAdverb(ref source) => source.is_lemma(),
            Initials(ref source) => source.is_lemma(),
            Shaped(ref source) => source.is_lemma(),
            Unknown(ref source) => source.is_lemma(),
        }
    }

    fn is_known(&self) -> bool {
        match *self {
            Dictionary(ref source) => source.is_known(),
            HyphenAdverb(ref source) => source.is_known(),
            Initials(ref source) => source.is_known(),
            Shaped(ref source) => source.is_known(),
            Unknown(ref source) => source.is_known(),
        }
    }

    fn get_word(&self) -> Cow<str> {
        match *self {
            Dictionary(ref source) => source.get_word(),
            HyphenAdverb(ref source) => source.get_word(),
            Initials(ref source) => source.get_word(),
            Shaped(ref source) => source.get_word(),
            Unknown(ref source) => source.get_word(),
        }
    }

    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str> {
        match *self {
            Dictionary(ref source) => source.get_normal_form(morph),
            HyphenAdverb(ref source) => source.get_normal_form(morph),
            Initials(ref source) => source.get_normal_form(morph),
            Shaped(ref source) => source.get_normal_form(morph),
            Unknown(ref source) => source.get_normal_form(morph),
        }
    }

    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        match *self {
            Dictionary(ref source) => source.get_tag(morph),
            HyphenAdverb(ref source) => source.get_tag(morph),
            Initials(ref source) => source.get_tag(morph),
            Shaped(ref source) => source.get_tag(morph),
            Unknown(ref source) => source.get_tag(morph),
        }
    }

    fn try_get_para_id(&self) -> Option<ParadigmId> {
        match *self {
            Dictionary(ref source) => source.try_get_para_id(),
            HyphenAdverb(ref source) => source.try_get_para_id(),
            Initials(ref source) => source.try_get_para_id(),
            Shaped(ref source) => source.try_get_para_id(),
            Unknown(ref source) => source.try_get_para_id(),
        }
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match *self {
            Dictionary(ref source) => source.write_word(f),
            HyphenAdverb(ref source) => source.write_word(f),
            Initials(ref source) => source.write_word(f),
            Shaped(ref source) => source.write_word(f),
            Unknown(ref source) => source.write_word(f),
        }
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        match *self {
            Dictionary(ref source) => source.write_normal_form(f, morph),
            HyphenAdverb(ref source) => source.write_normal_form(f, morph),
            Initials(ref source) => source.write_normal_form(f, morph),
            Shaped(ref source) => source.write_normal_form(f, morph),
            Unknown(ref source) => source.write_normal_form(f, morph),
        }
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        match *self {
            Dictionary(ref source) => source.get_lexeme(morph),
            HyphenAdverb(ref source) => source.get_lexeme(morph),
            Initials(ref source) => source.get_lexeme(morph),
            Shaped(ref source) => source.get_lexeme(morph),
            Unknown(ref source) => source.get_lexeme(morph),
        }
    }

    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        match *self {
            Dictionary(ref source) => source.get_lemma(morph),
            HyphenAdverb(ref source) => source.get_lemma(morph),
            Initials(ref source) => source.get_lemma(morph),
            Shaped(ref source) => source.get_lemma(morph),
            Unknown(ref source) => source.get_lemma(morph),
        }
    }
}

impl MorphySerde for StackSource {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match *self {
            Dictionary(ref source) => source.encode(f),
            HyphenAdverb(ref source) => source.encode(f),
            Initials(ref source) => source.encode(f),
            Shaped(ref source) => source.encode(f),
            Unknown(ref source) => source.encode(f),
        }
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        Ok(match try_decode::<Dictionary>(s)? {
            Some(v) => v,
            None => match try_decode::<HyphenAdverb>(s)? {
                Some(v) => v,
                None => match try_decode::<Initials>(s)? {
                    Some(v) => v,
                    None => match try_decode::<Shaped>(s)? {
                        Some(v) => v,
                        None => match try_decode::<Unknown>(s)? {
                            Some(v) => v,
                            None => Err(DecodeError::UnknownPartType)?,
                        },
                    },
                },
            },
        })
    }
}

fn try_decode<T: MorphySerde + Into<StackSource>>(
    s: &str,
) -> Result<Option<(&str, StackSource)>, DecodeError> {
    Ok(match T::decode(s) {
        Err(DecodeError::UnknownPartType) => None,
        Err(e) => Err(e)?,
        Ok((s, v)) => Some((s, v.into())),
    })
}
