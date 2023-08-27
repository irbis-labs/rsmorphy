use std::fmt;

use crate::analyzer::model::abc::*;
use crate::analyzer::model::affix::Affix;
use crate::analyzer::model::affix::AffixKind;
use crate::analyzer::model::decode::*;
use crate::analyzer::model::lexeme::Lexeme;
use crate::analyzer::model::stack::StackSource;
use crate::analyzer::model::Lex;
use crate::analyzer::model::Score;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::tag::TagReg;

#[derive(Debug, Clone, PartialEq)]
pub struct StackAffix {
    pub(in crate::analyzer) stack: StackSource,
    pub(in crate::analyzer) affix: Option<Affix>,
}

impl StackAffix {
    #[inline(always)]
    pub fn new<S, A>(stack: S, affix: A) -> Self
    where
        S: Into<StackSource>,
        A: Into<Option<Affix>>,
    {
        let stack = stack.into();
        let affix = affix.into();
        StackAffix { stack, affix }
    }

    #[inline(always)]
    pub fn stack(&self) -> &StackSource {
        &self.stack
    }

    #[inline(always)]
    pub fn affix(&self) -> Option<&Affix> {
        self.affix.as_ref()
    }

    #[inline(always)]
    pub fn into_parts(self) -> (StackSource, Option<Affix>) {
        (self.stack, self.affix)
    }
}

impl From<StackSource> for StackAffix {
    fn from(stack: StackSource) -> Self {
        StackAffix { stack, affix: None }
    }
}

impl Source for StackAffix {
    #[inline(always)]
    fn score(&self) -> Score {
        match &self.affix {
            None => self.stack.score(),
            Some(_affix) => {
                // self.stack.score();
                unimplemented!()
            }
        }
    }

    #[inline(always)]
    fn is_lemma(&self) -> bool {
        match &self.affix {
            None => self.stack.is_lemma(),
            Some(_affix) => self.stack.is_lemma(),
        }
    }

    #[inline(always)]
    fn is_known(&self) -> bool {
        match &self.affix {
            None => self.stack.is_known(),
            Some(affix) => self.stack.is_known() && affix.is_known(),
        }
    }

    #[inline(always)]
    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m TagReg {
        self.stack.get_tag(morph)
    }

    #[inline(always)]
    fn try_get_para_id(&self) -> Option<ParadigmId> {
        self.stack.try_get_para_id()
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        if let Some(affix) = &self.affix {
            if !affix.is_known_suffix() {
                f.write_str(affix.part())?;
            };
        }
        self.stack.write_word(f)
    }

    #[inline(always)]
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        if let Some(ref affix) = self.affix {
            if !affix.is_known_suffix() {
                f.write_str(affix.part())?;
            };
        }
        self.stack.write_normal_form(f, morph)
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

impl StackAffix {
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        let is_known_suffix = self
            .affix
            .as_ref()
            .map(Affix::is_known_suffix)
            .unwrap_or(false);

        let make_affix = move |new_source: &StackSource| -> Option<Affix> {
            if is_known_suffix {
                let dict_word = new_source.as_dictionary().unwrap_or_else(|| {
                    panic!(
                        "StackAffix::iter_lexeme: \
                         expected StackSource::Dictionary, \
                         got {:?}",
                        new_source
                    )
                });
                let word = dict_word.word_lower().word();
                let stem = morph
                    .dict()
                    .get_stem(dict_word.para_id(), dict_word.idx(), word);
                Some(Affix::known_suffix(&word[stem.len()..]))
            } else {
                self.affix.clone()
            }
        };

        self.stack.iter_lexeme(morph).map(move |lex: Lex| {
            let (left, _right) = lex.stack.stack.into_parts();
            let stack = left.stack;
            let affix = make_affix(&stack);
            Lex::from_stack(StackAffix::new(stack, affix))
        })
    }
}

impl MorphySerde for StackAffix {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        self.stack.encode(f)?;
        if let Some(ref affix) = self.affix {
            let kind = match affix.kind() {
                AffixKind::KnownPrefix => "kp",
                AffixKind::UnknownPrefix => "up",
                AffixKind::KnownSuffix => "ks",
            };
            let part = affix.part();
            write!(f, ";{kind}:{part}")?;
        }
        Ok(())
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let (s, source) = StackSource::decode(s)?;
        let mut result = (s, StackAffix::new(source, None));
        if !s.is_empty() {
            let parse = |s| {
                let s = follow_str(s, ";")?;
                let (s, kind) = follow_str(s, "ks")
                    .map(|s| (s, AffixKind::KnownSuffix))
                    .or_else(|_| follow_str(s, "kp").map(|s| (s, AffixKind::KnownPrefix)))
                    .or_else(|_| follow_str(s, "up").map(|s| (s, AffixKind::UnknownPrefix)))
                    .map_err(|e| match e {
                        DecodeError::DoesntMatch => DecodeError::UnknownPartType,
                        e => e,
                    })?;
                let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;
                Ok((s, Affix::new(word, kind)))
            };
            match parse(s) {
                Err(DecodeError::UnknownPartType) => (),
                Err(e) => Err(e)?,
                Ok((s, affix)) => {
                    result.0 = s;
                    result.1.affix = Some(affix);
                }
            };
        }
        Ok(result)
    }
}
