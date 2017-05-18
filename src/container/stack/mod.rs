use std::borrow::Cow;
use std::fmt;

use ::analyzer::MorphAnalyzer;
use ::container::abc::*;
use ::container::Lex;
use ::container::Score;
use ::opencorpora::OpencorporaTagReg;


pub mod affix;
pub mod hsp;
pub mod hword;
pub mod source;

pub use self::affix::StackAffix;
pub use self::hsp::StackParticle;
pub use self::hword::StackHyphenated;
pub use self::source::StackSource;


/*
    Ожидаемые инварианты стеков:
    * Dictinary[, KnownPrefix|UnknownPrefix][, HyphenSeparatedParticle]
    * FakeDictinary[, KnownSuffix][, HyphenSeparatedParticle]
    * Shaped[, HyphenSeparatedParticle]
    * Unknown[, HyphenSeparatedParticle]
    * Initials
*/

// TODO remove Stack and use StackParticle instead

#[derive(Debug, Clone, PartialEq)]
pub enum Stack {
    HSP(StackParticle),
    Hyphenated(StackHyphenated),
    Affix(StackAffix),
    Source(StackSource)
}


impl From<StackHyphenated> for Stack {
    fn from(v: StackHyphenated) -> Self { Stack::Hyphenated(v) }
}

impl From<StackParticle> for Stack {
    fn from(v: StackParticle) -> Self { Stack::HSP(v) }
}

impl From<StackAffix> for Stack {
    fn from(v: StackAffix) -> Self { Stack::Affix(v) }
}

impl From<StackSource> for Stack {
    fn from(v: StackSource) -> Self { Stack::Source(v) }
}


impl Source for Stack {
    fn score(&self) -> Score {
        match *self {
            Stack::HSP(ref stack)           => stack.score(),
            Stack::Hyphenated(ref stack)    => stack.score(),
            Stack::Affix(ref stack)         => stack.score(),
            Stack::Source(ref stack)        => stack.score(),
        }
    }

    fn is_lemma(&self) -> bool {
        match *self {
            Stack::HSP(ref stack)           => stack.is_lemma(),
            Stack::Hyphenated(ref stack)    => stack.is_lemma(),
            Stack::Affix(ref stack)         => stack.is_lemma(),
            Stack::Source(ref stack)        => stack.is_lemma(),
        }
    }

    fn is_known(&self) -> bool {
        match *self {
            Stack::HSP(ref stack)           => stack.is_known(),
            Stack::Hyphenated(ref stack)    => stack.is_known(),
            Stack::Affix(ref stack)         => stack.is_known(),
            Stack::Source(ref stack)        => stack.is_known(),
        }
    }

    fn get_word(&self) -> Cow<str> {
        match *self {
            Stack::HSP(ref stack)           => stack.get_word(),
            Stack::Hyphenated(ref stack)    => stack.get_word(),
            Stack::Affix(ref stack)         => stack.get_word(),
            Stack::Source(ref stack)        => stack.get_word(),
        }
    }

    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str> {
        match *self {
            Stack::HSP(ref stack)           => stack.get_normal_form(morph),
            Stack::Hyphenated(ref stack)    => stack.get_normal_form(morph),
            Stack::Affix(ref stack)         => stack.get_normal_form(morph),
            Stack::Source(ref stack)        => stack.get_normal_form(morph),
        }
    }

    fn get_tag<'a>(&self, morph: &'a MorphAnalyzer) -> &'a OpencorporaTagReg {
        match *self {
            Stack::HSP(ref stack)           => stack.get_tag(morph),
            Stack::Hyphenated(ref stack)    => stack.get_tag(morph),
            Stack::Affix(ref stack)         => stack.get_tag(morph),
            Stack::Source(ref stack)        => stack.get_tag(morph),
        }
    }

    fn try_get_para_id(&self) -> Option<u16> {
        match *self {
            Stack::HSP(ref stack)           => stack.try_get_para_id(),
            Stack::Hyphenated(ref stack)    => stack.try_get_para_id(),
            Stack::Affix(ref stack)         => stack.try_get_para_id(),
            Stack::Source(ref stack)        => stack.try_get_para_id(),
        }
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match *self {
            Stack::HSP(ref stack)           => stack.write_word(f),
            Stack::Hyphenated(ref stack)    => stack.write_word(f),
            Stack::Affix(ref stack)         => stack.write_word(f),
            Stack::Source(ref stack)        => stack.write_word(f),
        }
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        match *self {
            Stack::HSP(ref stack)           => stack.write_normal_form(f, morph),
            Stack::Hyphenated(ref stack)    => stack.write_normal_form(f, morph),
            Stack::Affix(ref stack)         => stack.write_normal_form(f, morph),
            Stack::Source(ref stack)        => stack.write_normal_form(f, morph),
        }
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        match *self {
            Stack::HSP(ref stack)           => stack.get_lexeme(morph),
            Stack::Hyphenated(ref stack)    => stack.get_lexeme(morph),
            Stack::Affix(ref stack)         => stack.get_lexeme(morph),
            Stack::Source(ref stack)        => stack.get_lexeme(morph),
        }
    }

    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        match *self {
            Stack::HSP(ref stack)           => stack.get_lemma(morph),
            Stack::Hyphenated(ref stack)    => stack.get_lemma(morph),
            Stack::Affix(ref stack)         => stack.get_lemma(morph),
            Stack::Source(ref stack)        => stack.get_lemma(morph),
        }
    }
}


impl Stack {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(&'s self, morph: &'m MorphAnalyzer) -> impl Iterator<Item = Lex> + 'i {
        let i: Box<Iterator<Item = Lex> + 'i> = match *self {
            Stack::HSP(ref stack)           => Box::new(stack.iter_lexeme(morph)),
            Stack::Hyphenated(ref stack)    => Box::new(stack.iter_lexeme(morph)),
            Stack::Affix(ref stack)         => Box::new(stack.iter_lexeme(morph)),
            Stack::Source(ref stack)        => Box::new(stack.iter_lexeme(morph)),
        };
        i.into_iter()
    }
}


impl MorphySerde for Stack {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match *self {
            Stack::HSP(ref stack)           => stack.encode(f),
            Stack::Hyphenated(ref stack)    => stack.encode(f),
            Stack::Affix(ref stack)         => stack.encode(f),
            Stack::Source(ref stack)        => stack.encode(f),
        }
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let (s, stack) = StackParticle::decode(s)?;
        Ok((s, stack.into()))
    }
}


impl Stack {
    pub fn try_as_hsp(&self) -> Option<&StackParticle> {
        match *self {
            Stack::HSP(ref stack) => Some(stack),
            _ => None
        }
    }

    pub fn try_as_hword(&self) -> Option<&StackHyphenated> {
        match *self {
            Stack::Hyphenated(ref stack) => Some(stack),
            _ => None
        }
    }

    pub fn try_as_affix(&self) -> Option<&StackAffix> {
        match *self {
            Stack::Affix(ref stack) => Some(stack),
            _ => None
        }
    }

    pub fn try_as_source(&self) -> Option<&StackSource> {
        match *self {
            Stack::Source(ref stack) => Some(stack),
            _ => None
        }
    }
}
