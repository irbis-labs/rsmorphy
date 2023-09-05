use std::fmt;

use inlinable_string::InlinableString;
use inlinable_string::StringExt;

use crate::analyzer::common::Base;
use crate::analyzer::common::HyphenSeparatedParticle;
use crate::analyzer::common::Main;
use crate::analyzer::Lang;
use crate::model::Dictionary;
use crate::GramSet;

#[derive(Debug, Clone)]
pub struct Lex {
    pub(in crate::analyzer) left: Main,
    pub(in crate::analyzer) right: Option<Main>,
    pub(in crate::analyzer) particle: Option<HyphenSeparatedParticle>,
}

#[derive(Debug, Clone, Copy)]
pub struct LexLower<'l>(&'l Lex);

#[derive(Debug, Clone, Copy)]
pub struct LexNormalForm<'l, 'd> {
    lex: &'l Lex,
    dict: &'d Dictionary,
}

impl Lex {
    #[inline(always)]
    pub fn new(left: Main, right: Option<Main>, particle: Option<HyphenSeparatedParticle>) -> Self {
        Lex {
            left,
            right,
            particle,
        }
    }

    #[inline(always)]
    pub fn from_base(left: impl Into<Base>) -> Self {
        let left = Main::new(left, None);
        Lex::new(left, None, None)
    }

    #[inline(always)]
    pub fn from_main(left: Main) -> Self {
        Lex::new(left, None, None)
    }

    #[inline(always)]
    pub fn is_known(&self) -> bool {
        self.left.is_known()
            && self
                .right
                .as_ref()
                .map(|right| right.is_known())
                .unwrap_or(true)
    }

    pub fn word_lower(&self) -> LexLower {
        LexLower(self)
    }

    pub fn make_word_lower(&self) -> InlinableString {
        let mut word = InlinableString::new();
        // SAFETY: this is safe because we are writing to an extendable buffer.
        self.write_word_lower(&mut word).unwrap();
        word
    }

    pub fn write_word_lower(&self, writer: &mut dyn fmt::Write) -> fmt::Result {
        self.left.write_word_lower(writer)?;
        if let Some(right) = &self.right {
            writer.write_str("-")?;
            right.write_word_lower(writer)?;
        }
        if let Some(particle) = &self.particle {
            writer.write_str(&particle.particle)?;
        }
        Ok(())
    }

    pub fn normal_form<'l, 'd>(&'l self, dict: &'d Dictionary) -> LexNormalForm<'l, 'd> {
        LexNormalForm { lex: self, dict }
    }

    pub fn make_normal_form(&self, dict: &Dictionary) -> InlinableString {
        let mut word = InlinableString::new();
        // SAFETY: this is safe because we are writing to an extendable buffer.
        self.write_normal_form(&mut word, dict).unwrap();
        word
    }

    pub fn write_normal_form(&self, writer: &mut dyn fmt::Write, dict: &Dictionary) -> fmt::Result {
        self.left.write_normal_form(writer, dict)?;
        if let Some(right) = &self.right {
            writer.write_str("-")?;
            right.write_normal_form(writer, dict)?;
        }
        if let Some(particle) = &self.particle {
            writer.write_str(&particle.particle)?;
        }
        Ok(())
    }

    /// The number should be passed similar to `(number.abs() % 100) as u8`.
    #[inline(always)]
    pub fn find_plural<L: Lang>(&self, morph: &L, num: u8) -> Option<Lex> {
        let tag = morph.get_tag(self);
        let required = morph
            .dict()
            .get_numeral_agreement_grammemes(tag.grammemes, num);
        self.inflect(morph, required)
    }

    pub fn inflect<L: Lang>(&self, morph: &L, mut required: GramSet) -> Option<Lex> {
        #[cfg(feature = "trace_analyzer")]
        {
            morph.iter_lexeme(self).for_each(|lex| {
                let tag = morph.get_tag(&lex);
                log::trace!(
                    "inflect: {} {},  present: {:?}",
                    lex.word_lower(),
                    tag.fmt_int,
                    tag.grammemes
                );
            });

            let tag = morph.get_tag(self);
            log::trace!(
                "inflect: {} {},  required: {:?}",
                self.word_lower(),
                tag.fmt_int,
                required
            );
        }

        let mut possible_results_iter = morph
            .iter_lexeme(self)
            .filter(|lex| morph.get_tag(lex).grammemes.is_superset(required))
            .peekable();

        if possible_results_iter.peek().is_some() {
            return self.complete_inflect(morph, required, &mut possible_results_iter);
        }

        required = morph.dict().fix_rare_cases(required);

        #[cfg(feature = "trace_analyzer")]
        log::trace!("inflect / fix rare_cases: {:?} {:?}", self, required);

        let mut possible_results_iter = morph
            .iter_lexeme(self)
            .filter(|lex| morph.get_tag(lex).grammemes.is_superset(required));

        let res = self.complete_inflect(morph, required, &mut possible_results_iter);

        #[cfg(feature = "trace_analyzer")]
        log::trace!("inflect / complete: {:?} {:?}", self, res);

        res
    }

    fn complete_inflect<L: Lang>(
        &self,
        morph: &L,
        required: GramSet,
        lexeme_iter: &mut dyn Iterator<Item = Lex>,
    ) -> Option<Lex> {
        #[inline(always)]
        fn similarity(target: GramSet, candidate: GramSet) -> i32 {
            let intersect_count = (target & candidate).len() as i32;
            let sym_diff_count = (target ^ candidate).len() as i32;
            intersect_count * 10 - sym_diff_count
        }

        #[cfg(feature = "trace_analyzer")]
        log::trace!("complete_inflect: {:?} {:?}", self, required);

        let mut target = morph.get_tag(self).grammemes;
        target = morph.dict().update_grammemes(target, required);

        #[cfg(feature = "trace_analyzer")]
        log::trace!("complete_inflect / target: {:?}", target);

        let mut best_result = None;
        let mut best_score = i32::MIN;
        for lex in lexeme_iter {
            let candidate = morph.get_tag(&lex).grammemes;
            let score = similarity(target, candidate);
            if score > best_score {
                best_result = Some(lex);
                best_score = score;

                #[cfg(feature = "trace_analyzer")]
                log::trace!(
                    "complete_inflect / candidate: {:?} {:?},  score / best: {} {}",
                    best_result,
                    candidate,
                    score,
                    best_score
                );
            } else {
                #[cfg(feature = "trace_analyzer")]
                log::trace!(
                    "complete_inflect / candidate: {:?} {:?},  score / best: {} {}",
                    lex,
                    candidate,
                    score,
                    best_score
                );
            }
        }

        #[cfg(feature = "trace_analyzer")]
        log::trace!("complete_inflect / best_result: {:?}", best_result);

        best_result
    }
}

impl fmt::Display for LexLower<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.write_word_lower(f)
    }
}

impl fmt::Display for LexNormalForm<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.lex.write_normal_form(f, self.dict)
    }
}
