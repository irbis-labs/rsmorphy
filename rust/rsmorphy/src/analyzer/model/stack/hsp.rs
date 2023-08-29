use std::fmt;

use crate::analyzer::model::abc::*;
use crate::analyzer::model::decode::*;
use crate::analyzer::model::lexeme::Lexeme;
use crate::analyzer::model::stack::StackAffix;
use crate::analyzer::model::stack::StackHyphenated;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::HyphenSeparatedParticle;
use crate::analyzer::model::Lex;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::tag::TagReg;

#[derive(Debug, Clone, PartialEq)]
pub struct StackParticle {
    pub(in crate::analyzer) stack: StackHyphenated,
    pub(in crate::analyzer) particle: Option<HyphenSeparatedParticle>,
}

impl StackParticle {
    #[inline(always)]
    pub fn new<P>(stack: StackHyphenated, particle: P) -> StackParticle
    where
        P: Into<Option<HyphenSeparatedParticle>>,
    {
        let particle = particle.into();
        StackParticle { stack, particle }
    }

    #[inline(always)]
    pub fn stack(&self) -> &StackHyphenated {
        &self.stack
    }

    #[inline(always)]
    pub fn particle(&self) -> Option<&HyphenSeparatedParticle> {
        self.particle.as_ref()
    }

    #[inline(always)]
    pub fn into_parts(self) -> (StackHyphenated, Option<HyphenSeparatedParticle>) {
        (self.stack, self.particle)
    }

    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        self.stack.iter_lexeme(morph).map(move |lex: Lex| {
            Lex::from_stack(StackParticle::new(lex.stack.stack, self.particle.clone()))
        })
    }
}

impl From<StackHyphenated> for StackParticle {
    fn from(stack: StackHyphenated) -> Self {
        StackParticle {
            stack,
            particle: None,
        }
    }
}

impl From<StackAffix> for StackParticle {
    fn from(stack: StackAffix) -> Self {
        StackHyphenated::from(stack).into()
    }
}

impl From<StackSource> for StackParticle {
    fn from(stack: StackSource) -> Self {
        StackAffix::from(stack).into()
    }
}

impl Source for StackParticle {
    #[inline(always)]
    fn is_lemma(&self) -> bool {
        self.stack.is_lemma()
    }

    #[inline(always)]
    fn is_known(&self) -> bool {
        self.stack.is_known()
    }

    #[inline(always)]
    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m TagReg {
        self.stack.get_tag(morph)
    }

    #[inline(always)]
    fn try_get_para_id(&self) -> Option<ParadigmId> {
        self.stack.try_get_para_id()
    }

    #[inline(always)]
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        self.stack.write_word(f)?;
        if let Some(ref particle) = self.particle {
            f.write_str(&particle.particle)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        self.stack.write_normal_form(f, morph)?;
        if let Some(ref particle) = self.particle {
            f.write_str(&particle.particle)?;
        }
        Ok(())
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

impl MorphySerde for StackParticle {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        self.stack.encode(f)?;
        if let Some(ref particle) = self.particle {
            write!(f, ";hp:{}", particle.particle)?;
        }
        Ok(())
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let (s, stack) = StackHyphenated::decode(s)?;
        let particle = None;
        let mut result = (s, StackParticle { stack, particle });
        if !s.is_empty() {
            let parse = |s| {
                let s = follow_str(s, ";")?;
                let s = follow_str(s, "hp").map_err(|e| match e {
                    DecodeError::DoesntMatch => DecodeError::UnknownPartType,
                    e => e,
                })?;
                let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;
                Ok((s, HyphenSeparatedParticle::new(word)))
            };
            match parse(s) {
                Err(DecodeError::UnknownPartType) => (),
                Err(e) => Err(e)?,
                Ok((s, particle)) => {
                    result.0 = s;
                    result.1.particle = Some(particle);
                }
            };
        }
        Ok(result)
    }
}
