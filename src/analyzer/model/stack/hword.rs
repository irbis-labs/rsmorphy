use std::fmt;

use crate::analyzer::model::abc::*;
use crate::analyzer::model::lexeme::Lexeme;
use crate::analyzer::model::stack::StackAffix;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Score;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::OpencorporaTagReg;

#[derive(Debug, Clone, PartialEq)]
pub struct StackHyphenated {
    pub(in crate::analyzer) left: StackAffix,
    pub(in crate::analyzer) right: Option<StackAffix>,
}

impl StackHyphenated {
    #[inline(always)]
    pub fn new<R>(left: StackAffix, right: R) -> Self
    where
        R: Into<Option<StackAffix>>,
    {
        let right = right.into();
        StackHyphenated { left, right }
    }

    #[inline(always)]
    pub fn stack(&self) -> &StackAffix {
        &self.left
    }

    #[inline(always)]
    pub fn right(&self) -> Option<&StackAffix> {
        self.right.as_ref()
    }

    #[inline(always)]
    pub fn into_parts(self) -> (StackAffix, Option<StackAffix>) {
        (self.left, self.right)
    }

    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        self.left.iter_lexeme(morph).map(move |lex: Lex| {
            Lex::from_stack(StackHyphenated::new(lex.stack.stack().left.clone(), None))
        })
    }
}

impl From<StackAffix> for StackHyphenated {
    fn from(stack: StackAffix) -> Self {
        StackHyphenated::new(stack, None)
    }
}

impl From<StackSource> for StackHyphenated {
    fn from(stack: StackSource) -> Self {
        StackAffix::from(stack).into()
    }
}

impl Source for StackHyphenated {
    #[inline(always)]
    fn score(&self) -> Score {
        unimplemented!()
    }

    #[inline(always)]
    fn is_lemma(&self) -> bool {
        match &self.right {
            None => self.left.is_lemma(),
            Some(right) => self.left.is_lemma() && right.is_lemma(),
        }
    }

    #[inline(always)]
    fn is_known(&self) -> bool {
        match &self.right {
            None => self.left.is_known(),
            Some(right) => self.left.is_known() && right.is_known(),
        }
    }

    #[inline(always)]
    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        match self.right {
            None => self.left.get_tag(morph),
            Some(_) => unimplemented!(),
        }
    }

    #[inline(always)]
    fn try_get_para_id(&self) -> Option<ParadigmId> {
        match self.right {
            None => self.left.try_get_para_id(),
            Some(_) => unimplemented!(),
        }
    }

    #[inline(always)]
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        self.left.write_word(f)?;
        if let Some(ref right) = self.right {
            f.write_str("-")?;
            right.write_word(f)?;
        }
        Ok(())
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        self.left.write_normal_form(f, morph)?;
        if let Some(ref right) = self.right {
            f.write_str("-")?;
            right.write_normal_form(f, morph)?;
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

impl MorphySerde for StackHyphenated {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        match self.right {
            Some(ref right) => {
                f.write_str("hw:")?;
                self.left.encode(f)?;
                f.write_str(";-")?;
                right.encode(f)
            }
            None => self.left.encode(f),
        }
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let (s, stack) = StackAffix::decode(s)?;
        let result = (s, StackHyphenated::new(stack, None));
        // TODO
        //        if !s.is_empty() {
        //            match (|s| {
        //                let s = follow_str(s, ";")?;
        //                let s = follow_str(s, "hw").map_err(|e| match e {
        //                    DecodeError::DoesntMatch => DecodeError::UnknownPartType,
        //                    _ => e,
        //                })?;
        //                let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;
        //                Ok((s, HyphenSeparatedParticle {
        //                    particle: word.to_string(),
        //                }))
        //            })(s) {
        //                Err(DecodeError::UnknownPartType) => (),
        //                Err(e) => Err(e)?,
        //                Ok((s, particle)) => {
        //                    result.0 = s;
        //                    result.1.particle = Some(particle);
        //                },
        //            };
        //        }
        Ok(result)
    }
}
