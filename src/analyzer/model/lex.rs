use std::fmt;

use crate::analyzer::model::abc::*;
use crate::analyzer::model::decode::*;
use crate::analyzer::model::lexeme::Lexeme;
use crate::analyzer::model::stack::StackParticle;
use crate::analyzer::model::Score;
use crate::analyzer::model::Seen;
use crate::analyzer::MorphAnalyzer;
use crate::opencorpora::dictionary::ParadigmId;
use crate::opencorpora::GrammemeSet;
use crate::opencorpora::OpencorporaTagReg;

#[derive(Debug, Clone, PartialEq)]
pub struct Lex {
    pub(in crate::analyzer) stack: StackParticle,
}

impl Lex {
    #[inline(always)]
    pub fn from_id<S>(id: S) -> Result<Self, DecodeError>
    where
        S: AsRef<str>,
    {
        Ok(Self::decode(id.as_ref()).map(|(_, lex)| lex)?)
    }

    #[inline(always)]
    pub fn from_stack<S>(stack: S) -> Self
    where
        S: Into<StackParticle>,
    {
        let stack = stack.into();
        Lex { stack }
    }

    pub fn stack(&self) -> &StackParticle {
        &self.stack
    }

    pub fn into_inner(self) -> StackParticle {
        self.stack
    }

    #[inline(always)]
    pub fn iter_lexeme<'s: 'i, 'm: 'i, 'i>(
        &'s self,
        morph: &'m MorphAnalyzer,
    ) -> impl Iterator<Item = Lex> + 'i {
        self.stack.iter_lexeme(morph)
    }

    #[inline(always)]
    pub fn to_seen<'m>(&self, morph: &'m MorphAnalyzer) -> Seen<'m> {
        Seen {
            word: self.get_word(),
            tag: self.get_tag(morph),
            para_id: self.stack.try_get_para_id(),
        }
    }

    #[inline(always)]
    pub fn get_plural(&self, morph: &MorphAnalyzer, num: usize) -> Option<Lex> {
        self.inflect(morph, &self.get_tag(morph).numeral_agreement_grammemes(num))
    }

    pub fn inflect(&self, morph: &MorphAnalyzer, required_grammemes: &GrammemeSet) -> Option<Lex> {
        let mut possible_results_iter = self
            .iter_lexeme(morph)
            .filter(|lex| lex.get_tag(morph).grammemes.is_superset(required_grammemes))
            .peekable();

        if possible_results_iter.peek().is_some() {
            return self.complete_inflect(morph, required_grammemes, &mut possible_results_iter);
        }

        let mut required_grammemes = required_grammemes.clone();
        morph.dict().fix_rare_cases(&mut required_grammemes);

        let mut possible_results_iter = self.iter_lexeme(morph).filter(|lex| {
            lex.get_tag(morph)
                .grammemes
                .is_superset(&required_grammemes)
        });

        self.complete_inflect(morph, &required_grammemes, &mut possible_results_iter)
    }

    fn complete_inflect(
        &self,
        morph: &MorphAnalyzer,
        required_grammemes: &GrammemeSet,
        lexeme_iter: &mut dyn Iterator<Item = Lex>,
    ) -> Option<Lex> {
        fn similarity(morph: &MorphAnalyzer, grammemes: &GrammemeSet, lex: &Lex) -> i32 {
            let tag = lex.get_tag(morph);
            let intersect_count = grammemes.intersection(&tag.grammemes).count() as i32;
            let sym_diff_count = grammemes.symmetric_difference(&tag.grammemes).count() as i32;
            intersect_count * 10 - sym_diff_count
        }

        let mut grammemes = self.get_tag(morph).grammemes.clone();
        morph
            .dict()
            .update_grammemes(&mut grammemes, required_grammemes);

        let mut best_result = None;
        let mut best_score = i32::MIN;
        for lex in lexeme_iter {
            let score = similarity(morph, &grammemes, &lex);
            if score > best_score {
                best_result = Some(lex);
                best_score = score;
            }
        }
        best_result
    }
}

impl Source for Lex {
    #[inline(always)]
    fn score(&self) -> Score {
        self.stack.score()
    }

    #[inline(always)]
    fn is_lemma(&self) -> bool {
        self.stack.is_lemma()
    }

    #[inline(always)]
    fn is_known(&self) -> bool {
        self.stack.is_known()
    }

    #[inline(always)]
    fn get_tag<'m>(&self, morph: &'m MorphAnalyzer) -> &'m OpencorporaTagReg {
        self.stack.get_tag(morph)
    }

    #[inline(always)]
    fn try_get_para_id(&self) -> Option<ParadigmId> {
        self.stack.try_get_para_id()
    }

    #[inline(always)]
    fn write_word<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        self.stack.write_word(f)
    }

    #[inline(always)]
    fn write_normal_form<W: fmt::Write>(&self, f: &mut W, morph: &MorphAnalyzer) -> fmt::Result {
        self.stack.write_normal_form(f, morph)
    }

    #[inline(always)]
    fn get_lexeme(&self, morph: &MorphAnalyzer) -> Lexeme {
        self.stack.get_lexeme(morph)
    }

    #[inline(always)]
    fn get_lemma(&self, morph: &MorphAnalyzer) -> Lex {
        self.stack.get_lemma(morph)
    }
}

impl fmt::Display for Lex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_word(f)
    }
}

impl MorphySerde for Lex {
    fn encode<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str("ru:")?;
        self.stack.encode(f)
    }

    fn decode(s: &str) -> Result<(&str, Self), DecodeError> {
        let s = follow_str(s, "ru").map_err(|_| DecodeError::UnknownPartType)?;
        let (s, stack) = StackParticle::decode(follow_str(s, ":")?)?;
        Ok((s, Lex { stack }))
    }
}
