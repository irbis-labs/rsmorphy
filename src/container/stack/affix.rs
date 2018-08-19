use std::borrow::Cow;
use std::fmt;

use analyzer::MorphAnalyzer;
use container::abc::*;
use container::affix::{Affix, AffixKind};
use container::decode::*;
use container::paradigm::ParadigmId;
use container::stack::StackSource;
use container::{Lex, Score};
use opencorpora::OpencorporaTagReg;

#[derive(Debug, Clone, PartialEq)]
pub struct StackAffix {
    pub stack: StackSource,
    pub affix: Option<Affix>,
}

impl StackAffix {
    pub fn new<S, A>(stack: S, affix: A) -> Self
    where
        S: Into<StackSource>,
        A: Into<Option<Affix>>,
    {
        let stack = stack.into();
        let affix = affix.into();
        StackAffix { stack, affix }
    }
}

impl From<StackSource> for StackAffix {
    fn from(stack: StackSource) -> Self {
        StackAffix { stack, affix: None }
    }
}

impl Source for StackAffix {
    fn score(&self) -> Score {
        match self.affix {
            None => self.stack.score(),
            Some(ref _affix) => {
                // self.stack.score();
                unimplemented!()
            }
        }
    }

    fn is_lemma(&self) -> bool {
        match self.affix {
            None => self.stack.is_lemma(),
            Some(ref _affix) => self.stack.is_lemma(),
        }
    }

    fn is_known(&self) -> bool {
        match self.affix {
            None => self.stack.is_known(),
            Some(ref affix) => affix.is_known() && self.stack.is_known(),
        }
    }

    fn get_word(&self) -> Cow<str> {
        match self.affix {
            None => self.stack.get_word(),
            Some(ref affix) => match affix.kind {
                AffixKind::KnownSuffix => self.stack.get_word(),
                _ => format!("{}{}", affix.part, self.stack.get_word()).into(),
            },
        }
    }

    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str> {
        match self.affix {
            None => self.stack.get_normal_form(morph),
            Some(ref affix) => match affix.kind {
                AffixKind::KnownSuffix => self.stack.get_normal_form(morph),
                _ => format!("{}{}", affix.part, self.stack.get_normal_form(morph)).into(),
            },
        }
    }

    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        self.stack.get_tag(morph)
    }

    fn try_get_para_id(&self) -> Option<ParadigmId> {
        self.stack.try_get_para_id()
    }

    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        if let Some(ref affix) = self.affix {
            match affix.kind {
                AffixKind::KnownSuffix => (),
                _ => write!(f, "{}", affix.part)?,
            };
        }
        self.stack.write_word(f)
    }

    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        if let Some(ref affix) = self.affix {
            match affix.kind {
                AffixKind::KnownSuffix => (),
                _ => write!(f, "{}", affix.part)?,
            };
        }
        self.stack.write_normal_form(f, morph)
    }

    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Vec<Lex> {
        self.iter_lexeme(morph).collect()
    }

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
                let dict = new_source
                    .as_dictionary()
                    .expect("Should only be Dictionary");
                let word = dict.word_lower().word();
                let stem = morph.dict.get_stem(dict.para_id(), dict.idx(), word);
                Some(Affix::known_suffix(&word[stem.len()..]))
            } else {
                self.affix.clone()
            }
        };

        self.stack.iter_lexeme(morph).map(move |lex: Lex| {
            let stack = lex.stack.stack.left.stack;
            let affix = make_affix(&stack);
            Lex::from_stack(morph, StackAffix::new(stack, affix))
        })
    }
}

impl MorphySerde for StackAffix {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        self.stack.encode(f)?;
        if let Some(ref affix) = self.affix {
            write!(
                f,
                ";{}:{}",
                match affix.kind {
                    AffixKind::KnownPrefix => "kp",
                    AffixKind::UnknownPrefix => "up",
                    AffixKind::KnownSuffix => "ks",
                },
                affix.part
            )?;
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
