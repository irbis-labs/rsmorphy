use std::borrow::Cow;
use std::fmt;

use analyzer::MorphAnalyzer;
use container::abc::*;
use container::Lex;
use container::Score;
use container::affix::Affix;
use container::affix::AffixKind;
use container::stack::Stack;
use container::stack::StackSource;
use opencorpora::OpencorporaTagReg;

use container::decode::*;



#[derive(Debug, Clone, PartialEq)]
pub struct StackAffix {
    pub stack: StackSource,
    pub affix: Option<Affix>,
}


impl From<StackSource> for StackAffix {
    fn from(stack: StackSource) -> Self { StackAffix { stack: stack, affix: None } }
}


impl Source for StackAffix {
    fn score(&self) -> Score {
        match self.affix {
            None            => self.stack.score(),
            Some(ref _affix) => {
                self.stack.score();
                unimplemented!()
            },
        }
    }

    fn is_lemma(&self) -> bool {
        match self.affix {
            None            => self.stack.is_lemma(),
            Some(ref _affix) => {
                self.stack.is_lemma();
                unimplemented!()
            },
        }
    }

    fn is_known(&self) -> bool {
        match self.affix {
            None            => self.stack.is_known(),
            Some(ref affix) => {
                affix.is_known() && self.stack.is_known()
            },
        }
    }

    fn get_word(&self) -> Cow<str> {
        match self.affix {
            None            => self.stack.get_word(),
            Some(ref affix) =>
                match affix.kind {
                    AffixKind::KnownSuffix => self.stack.get_word(),
                    _ => format!("{}{}", affix.part, self.stack.get_word()).into(),
                },
        }
    }

    fn get_normal_form(&self, morph: &MorphAnalyzer) -> Cow<str> {
        match self.affix {
            None            => self.stack.get_normal_form(morph),
            Some(ref affix) =>
                match affix.kind {
                    AffixKind::KnownSuffix => self.stack.get_normal_form(morph),
                    _ => format!("{}{}", affix.part, self.stack.get_normal_form(morph)).into(),
                },

        }
    }

    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        self.stack.get_tag(morph)
    }

    fn try_get_para_id(&self) -> Option<u16> {
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
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(&'s self, morph: &'m MorphAnalyzer) -> impl Iterator<Item = Lex> + 'i {
        let stem = match self.affix {
            Some(ref affix) if affix.kind == AffixKind::KnownSuffix => true,
            _ => false,
        };
        let make_affix = move |new_source_stack: &StackSource| {
            if stem {
                Some(Affix {
                    kind: AffixKind::KnownSuffix,
                    part: match *new_source_stack {
                        StackSource::Dictionary(ref dic) => {
                            let stem = morph.dict.build_stem(dic.para_id, dic.idx, dic.word_lower.word.as_str());
                            dic.word_lower.word[stem.len()..].to_string()
                        }
                        _ => unreachable!(),
                    },
                })
            } else {
                self.affix.clone()
            }
        };

        self.stack.iter_lexeme(morph).map(move |lex: Lex| {
            let new_source_stack = match lex.stack {
                Stack::Source(stack) => stack,
                _ => unreachable!()
            };
            Lex {
                stack: StackAffix {
                    affix: make_affix(&new_source_stack),
                    stack: new_source_stack,
                }.into()
            }
        })

//        self.stack.iter_lexeme(morph).map(move |lex: Lex| {
//            let source_stack = match lex.stack {
//                Stack::Source(stack) => stack,
//                _ => unreachable!()
//            };
//            Lex {
//                stack: StackAffix {
//                    stack: source_stack,
//                    affix: match self.affix.kind {
//                        AffixKind::KnownSuffix => {
//                            Affix {
//                                kind: AffixKind::KnownSuffix,
//                                part: self.affix.part
//                            }
//                        },
//                        _ => self.affix.clone()
//                    },
//                }.into()
//            }
//        })
    }
}


impl MorphySerde for StackAffix {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        self.stack.encode(f)?;
        if let Some(ref affix) = self.affix {
            write!(
                f, ";{}:{}",
                match affix.kind {
                    AffixKind::KnownPrefix      => "kp",
                    AffixKind::UnknownPrefix    => "up",
                    AffixKind::KnownSuffix      => "ks",
                },
                affix.part
            )?;
        }
        Ok(())
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let (s, source) = StackSource::decode(s)?;
        let mut result = (s, StackAffix {
            stack: source,
            affix: None,
        });
        if !s.is_empty() {
            match (|s| {
                let s = follow_str(s, ";")?;
                let (s, kind) = follow_str(s, "ks").map(|s| (s, AffixKind::KnownSuffix))
                    .or_else(|_| follow_str(s, "kp").map(|s| (s, AffixKind::KnownPrefix)))
                    .or_else(|_| follow_str(s, "up").map(|s| (s, AffixKind::UnknownPrefix)))
                    .map_err(|e| match e {
                        DecodeError::DoesntMatch => DecodeError::UnknownPartType,
                        _ => e,
                    })?;
                let (s, word) = take_str_until_char_is(follow_str(s, ":")?, ';')?;
                Ok((s, Affix {
                    kind: kind,
                    part: word.to_string(),
                }))
            })(s) {
                Err(DecodeError::UnknownPartType) => (),
                Err(e) => Err(e)?,
                Ok((s, affix)) => {
                    result.0 = s;
                    result.1.affix = Some(affix);
                },
            };
        }
        Ok(result)
    }
}
