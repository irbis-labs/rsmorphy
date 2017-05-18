use std::borrow::Cow;
use std::fmt;

use ::analyzer::MorphAnalyzer;
use ::container::abc::*;
use ::container::Dictionary;
use ::container::Lex;
use ::container::HyphenAdverb;
use ::container::Initials;
use ::container::Shaped;
use ::container::Score;
use ::container::Unknown;
use ::opencorpora::OpencorporaTagReg;


#[derive(Debug, Clone, PartialEq)]
pub enum StackSource {
    Dictionary(Dictionary),
    HyphenAdverb(HyphenAdverb),
    Initials(Initials),
    Shaped(Shaped),
    Unknown(Unknown),
}


impl From<Dictionary> for StackSource {
    fn from(source: Dictionary) -> Self { StackSource::Dictionary(source) }
}

impl From<HyphenAdverb> for StackSource {
    fn from(source: HyphenAdverb) -> Self { StackSource::HyphenAdverb(source) }
}

impl From<Initials> for StackSource {
    fn from(source: Initials) -> Self { StackSource::Initials(source) }
}

impl From<Shaped> for StackSource {
    fn from(source: Shaped) -> Self { StackSource::Shaped(source) }
}

impl From<Unknown> for StackSource {
    fn from(source: Unknown) -> Self { StackSource::Unknown(source) }
}


impl Source for StackSource {
    fn score(&self) -> Score {
        match *self {
            StackSource::Dictionary(ref source)     => source.score(),
            StackSource::HyphenAdverb(ref source)   => source.score(),
            StackSource::Initials(ref source)       => source.score(),
            StackSource::Shaped(ref source)         => source.score(),
            StackSource::Unknown(ref source)        => source.score(),
        }
    }

    fn is_lemma(&self) -> bool {
        match *self {
            StackSource::Dictionary(ref source)     => source.is_lemma(),
            StackSource::HyphenAdverb(ref source)   => source.is_lemma(),
            StackSource::Initials(ref source)       => source.is_lemma(),
            StackSource::Shaped(ref source)         => source.is_lemma(),
            StackSource::Unknown(ref source)        => source.is_lemma(),
        }
    }

    fn is_known(&self) -> bool {
        match *self {
            StackSource::Dictionary(ref source)     => source.is_known(),
            StackSource::HyphenAdverb(ref source)   => source.is_known(),
            StackSource::Initials(ref source)       => source.is_known(),
            StackSource::Shaped(ref source)         => source.is_known(),
            StackSource::Unknown(ref source)        => source.is_known(),
        }
    }

    fn get_word(&self) -> Cow<str> {
        match *self {
            StackSource::Dictionary(ref source)     => source.get_word(),
            StackSource::HyphenAdverb(ref source)   => source.get_word(),
            StackSource::Initials(ref source)       => source.get_word(),
            StackSource::Shaped(ref source)         => source.get_word(),
            StackSource::Unknown(ref source)        => source.get_word(),
        }
    }

    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str> {
        match *self {
            StackSource::Dictionary(ref source)     => source.get_normal_form(morph),
            StackSource::HyphenAdverb(ref source)   => source.get_normal_form(morph),
            StackSource::Initials(ref source)       => source.get_normal_form(morph),
            StackSource::Shaped(ref source)         => source.get_normal_form(morph),
            StackSource::Unknown(ref source)        => source.get_normal_form(morph),
        }
    }

    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        match *self {
            StackSource::Dictionary(ref source)     => source.get_tag(morph),
            StackSource::HyphenAdverb(ref source)   => source.get_tag(morph),
            StackSource::Initials(ref source)       => source.get_tag(morph),
            StackSource::Shaped(ref source)         => source.get_tag(morph),
            StackSource::Unknown(ref source)        => source.get_tag(morph),
        }
    }

    fn try_get_para_id(&self) -> Option<u16> {
        match *self {
            StackSource::Dictionary(ref source)     => source.try_get_para_id(),
            StackSource::HyphenAdverb(ref source)   => source.try_get_para_id(),
            StackSource::Initials(ref source)       => source.try_get_para_id(),
            StackSource::Shaped(ref source)         => source.try_get_para_id(),
            StackSource::Unknown(ref source)        => source.try_get_para_id(),
        }
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match *self {
            StackSource::Dictionary(ref source)     => source.write_word(f),
            StackSource::HyphenAdverb(ref source)   => source.write_word(f),
            StackSource::Initials(ref source)       => source.write_word(f),
            StackSource::Shaped(ref source)         => source.write_word(f),
            StackSource::Unknown(ref source)        => source.write_word(f),
        }
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        match *self {
            StackSource::Dictionary(ref source)     => source.write_normal_form(f, morph),
            StackSource::HyphenAdverb(ref source)   => source.write_normal_form(f, morph),
            StackSource::Initials(ref source)       => source.write_normal_form(f, morph),
            StackSource::Shaped(ref source)         => source.write_normal_form(f, morph),
            StackSource::Unknown(ref source)        => source.write_normal_form(f, morph),
        }
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        match *self {
            StackSource::Dictionary(ref source)     => source.get_lexeme(morph),
            StackSource::HyphenAdverb(ref source)   => source.get_lexeme(morph),
            StackSource::Initials(ref source)       => source.get_lexeme(morph),
            StackSource::Shaped(ref source)         => source.get_lexeme(morph),
            StackSource::Unknown(ref source)        => source.get_lexeme(morph),
        }
    }

    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        match *self {
            StackSource::Dictionary(ref source)     => source.get_lemma(morph),
            StackSource::HyphenAdverb(ref source)   => source.get_lemma(morph),
            StackSource::Initials(ref source)       => source.get_lemma(morph),
            StackSource::Shaped(ref source)         => source.get_lemma(morph),
            StackSource::Unknown(ref source)        => source.get_lemma(morph),
        }
    }
}


impl StackSource {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(&'s self, morph: &'m MorphAnalyzer) -> impl Iterator<Item = Lex> + 'i {
        let i: Box<Iterator<Item = Lex> + 'i> = match *self {
            StackSource::Dictionary(ref source)     => Box::new(source.iter_lexeme(morph)),
            StackSource::HyphenAdverb(ref source)   => Box::new(source.iter_lexeme(morph)),
            StackSource::Initials(ref source)       => Box::new(source.iter_lexeme(morph)),
            StackSource::Shaped(ref source)         => Box::new(source.iter_lexeme(morph)),
            StackSource::Unknown(ref source)        => Box::new(source.iter_lexeme(morph)),
        };
        i.into_iter()
    }
}


impl MorphySerde for StackSource {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match *self {
            StackSource::Dictionary(ref source)     => source.encode(f),
            StackSource::HyphenAdverb(ref source)   => source.encode(f),
            StackSource::Initials(ref source)       => source.encode(f),
            StackSource::Shaped(ref source)         => source.encode(f),
            StackSource::Unknown(ref source)        => source.encode(f),
        }
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        Ok(match try_type::<Dictionary>(s)? {
            Some(v) => v,
            None => match try_type::<HyphenAdverb>(s)? {
                Some(v) => v,
                None => match try_type::<Initials>(s)? {
                    Some(v) => v,
                    None => match try_type::<Shaped>(s)? {
                        Some(v) => v,
                        None => match try_type::<Unknown>(s)? {
                            Some(v) => v,
                            None => Err(DecodeError::UnknownPartType)?,
                        },
                    }
                }
            }
        })
    }
}


fn try_type<T: MorphySerde + Into<StackSource>>(s: &str) -> Result<Option<(&str, StackSource)>, DecodeError> {
    Ok(match T::decode(s) {
        Err(DecodeError::UnknownPartType) => None,
        Err(e) => Err(e)?,
        Ok((s, v)) => Some((s, v.into())),
    })
}


impl StackSource {
    pub fn try_as_dictionary(&self) -> Option<&Dictionary> {
        match *self {
            StackSource::Dictionary(ref source) => Some(source),
            _ => None
        }
    }

    pub fn try_as_hyphen_adverb(&self) -> Option<&HyphenAdverb> {
        match *self {
            StackSource::HyphenAdverb(ref source) => Some(source),
            _ => None
        }
    }

    pub fn try_as_initials(&self) -> Option<&Initials> {
        match *self {
            StackSource::Initials(ref source) => Some(source),
            _ => None
        }
    }

    pub fn try_as_shaped(&self) -> Option<&Shaped> {
        match *self {
            StackSource::Shaped(ref source) => Some(source),
            _ => None
        }
    }

    pub fn try_as_unknown(&self) -> Option<&Unknown> {
        match *self {
            StackSource::Unknown(ref source) => Some(source),
            _ => None
        }
    }
}
