use std::collections::HashMap;
use std::fmt;

use inlinable_string::InlinableString;
use smallvec::SmallVec;
use string_cache::DefaultAtom;

use crate::dawg::CompletionDawg;
use crate::dawg::DictEntry;
use crate::dawg::GuideEntry;
use crate::dawg::HH;
use crate::model::dictionary::dictionary_inner::DictionaryInner;
use crate::model::dictionary::numeral_agreement::NumeralGrammemes;
use crate::model::dictionary::value::WordValue;
use crate::model::dictionary::ParadigmEntry;
use crate::model::dictionary::PredictionSuffixValue;
use crate::model::gram::Gram;
use crate::model::gram::GramRegistry;
use crate::model::gram::GramRegistryStorage;
use crate::model::gram::GramSet;
use crate::model::tag::TagReg;
use crate::model::ParadigmId;
use crate::model::ParadigmIndex;
use crate::model::Paradigms;

pub type PredictionSuffixesDawg<'d> = CompletionDawg<'d, PredictionSuffixValue>;

/// Open Corpora dictionary wrapper class.
#[derive(Debug)]
pub struct Dictionary {
    inner: DictionaryInner,
}

impl Dictionary {
    #[inline(always)]
    pub(in crate::model::dictionary) fn new(
        grammemes: GramRegistryStorage,
        gramtab: Box<[TagReg]>,
        suffixes: Box<[DefaultAtom]>,
        paradigms: (Box<[u32]>, Box<[ParadigmEntry]>),
        words: (Box<[DictEntry]>, Box<[GuideEntry]>),
        p_t_given_w: Option<(Box<[DictEntry]>, Box<[GuideEntry]>)>,
        prediction_suffixes_dawgs: Vec<(Box<[DictEntry]>, Box<[GuideEntry]>)>,
        paradigm_prefixes: SmallVec<[DefaultAtom; 4]>,
        max_suffix_length: u32,
        char_substitutes: HashMap<InlinableString, InlinableString>,
        rare_cases: SmallVec<[(Gram, Gram); 8]>,
        non_productive: GramSet,
        numeral_grammemes: NumeralGrammemes,
    ) -> Self {
        Dictionary {
            inner: DictionaryInner::new(
                grammemes,
                gramtab,
                suffixes,
                paradigms,
                words,
                p_t_given_w,
                prediction_suffixes_dawgs,
                paradigm_prefixes,
                max_suffix_length,
                char_substitutes,
                rare_cases,
                non_productive,
                numeral_grammemes,
            ),
        }
    }

    #[inline(always)]
    pub fn grammemes(&self) -> GramRegistry<'_> {
        self.inner.grammemes()
    }

    #[inline(always)]
    pub fn gramtab(&self) -> &[TagReg] {
        self.inner.gramtab()
    }

    #[inline(always)]
    pub fn suffixes(&self) -> &[DefaultAtom] {
        self.inner.suffixes()
    }

    #[inline(always)]
    pub fn paradigms(&self) -> Paradigms<'_> {
        self.inner.paradigms()
    }

    #[inline(always)]
    pub fn para(&self) -> Paradigms<'_> {
        self.inner.para()
    }

    #[inline(always)]
    pub fn words(&self) -> CompletionDawg<'_, WordValue> {
        self.inner.words()
    }

    #[inline(always)]
    pub fn p_t_given_w(&self) -> Option<CompletionDawg<HH>> {
        self.inner.p_t_given_w()
    }

    #[inline(always)]
    pub fn prediction_suffixes_dawgs(&self, n: usize) -> CompletionDawg<PredictionSuffixValue> {
        self.inner.prediction_suffixes_dawgs(n)
    }

    #[inline(always)]
    pub fn paradigm_prefixes_len(&self) -> usize {
        self.inner.paradigm_prefixes_len()
    }

    #[inline(always)]
    pub fn iter_paradigm_prefixes(
        &self,
    ) -> impl Iterator<Item = &str> + DoubleEndedIterator + ExactSizeIterator + '_ {
        self.inner.iter_paradigm_prefixes()
    }

    #[inline(always)]
    pub fn max_suffix_length(&self) -> usize {
        self.inner.max_suffix_length()
    }

    #[inline(always)]
    pub fn char_substitutes(&self) -> &HashMap<InlinableString, InlinableString> {
        self.inner.char_substitutes()
    }

    #[inline(always)]
    pub fn prediction_splits(&self, word_len: usize) -> impl Iterator<Item = usize> {
        (1..=self.max_suffix_length().min(word_len.saturating_sub(1))).rev()
    }

    /// Return tag as a string
    #[inline]
    pub fn get_tag(&self, id: ParadigmId, idx: ParadigmIndex) -> &TagReg {
        let para = self.para().get(id).expect("paradigm");
        &self.gramtab()[para[idx.index()].tag_id() as usize]
    }

    #[inline]
    pub fn iter_paradigm_info<'d: 'i, 'i>(
        &'d self,
        id: ParadigmId,
    ) -> impl Iterator<Item = (&'d str, &'d TagReg, &'d str)> + 'i {
        self.para()
            .get(id)
            .expect("paradigm")
            .into_iter()
            .map(move |entry: &'d ParadigmEntry| self.inner.paradigm_entry_info(*entry))
    }

    /// Return word stem (given a word, paradigm and the word index).
    #[inline]
    pub(crate) fn get_stem<'a>(
        &self,
        id: ParadigmId,
        idx: ParadigmIndex,
        fixed_word: &'a str,
    ) -> &'a str {
        let paradigm = self.para().get(id).expect("paradigm");
        let para_entry = paradigm[idx.index()];
        let (prefix, _, suffix) = self.inner.paradigm_entry_info(para_entry);
        let begin = prefix.len();
        let end = fixed_word.len() - suffix.len();
        &fixed_word[begin..end]
    }

    /// Write the normal form.
    pub fn write_normal_form(
        &self,
        mut f: impl fmt::Write,
        id: ParadigmId,
        idx: ParadigmIndex,
        fixed_word: &str,
    ) -> fmt::Result {
        if idx.is_first() {
            f.write_str(fixed_word)?;
        } else {
            let stem = self.get_stem(id, idx, fixed_word);
            let para = self.para().get(id).expect("paradigm");
            let (normal_prefix, _, normal_suffix) = self.inner.paradigm_entry_info(para[0]);
            f.write_str(normal_prefix)?;
            f.write_str(stem)?;
            f.write_str(normal_suffix)?;
        }
        Ok(())
    }

    pub fn update_grammemes(&self, grammemes: GramSet, required: GramSet) -> GramSet {
        required
            .iter()
            .fold(grammemes | required, |grammemes, gram| {
                grammemes - self.grammemes().get_reg_by_gram(gram).incompatible
            })
    }

    pub fn fix_rare_cases(&self, mut grammemes: GramSet) -> GramSet {
        for &(from, to) in self.inner.rare_cases() {
            if grammemes.contains(from) {
                grammemes.remove(from);
                grammemes.insert(to);
            }
        }
        grammemes
    }

    #[inline(always)]
    pub fn is_productive(&self, grammemes: GramSet) -> bool {
        (grammemes & self.inner.non_productive()).is_empty()
    }

    /// Returns grammemes that are required to be present in the word.
    /// The number should be passed similar to `(number.abs() % 100) as u8`.
    pub fn get_numeral_agreement_grammemes(&self, grammemes: GramSet, num: u8) -> GramSet {
        enum NK {
            One,
            Little,
            Many,
        }

        let NumeralGrammemes {
            noun,
            adjf,
            prtf,
            sing,
            plur,
            nomn,
            gent,
            accs,
            femn,
            case,
        } = *self.inner.numeral_grammemes();

        let (num_10, num_100) = (num % 10, num % 100);
        let num_kind = match () {
            () if (num_10 == 1) && (num_100 != 11) => NK::One,
            () if (num_10 >= 2) && (num_10 <= 4) && (num_100 < 10 || num_100 >= 20) => NK::Little,
            () => NK::Many,
        };

        let g = grammemes;
        match num_kind {
            _ if g.is_disjoint(noun | adjf | prtf) => GramSet::empty(),

            _ if g.is_superset(noun) && g.is_disjoint(nomn | accs) => {
                (case & g) | if let NK::One = num_kind { sing } else { plur }
            }

            NK::One if g.is_superset(nomn) => sing | nomn,
            NK::One => sing | accs,

            NK::Little if g.is_superset(noun) => sing | gent,
            NK::Little if g.is_superset(femn) && g.is_intersect(adjf | prtf) => plur | nomn,

            _ => plur | gent,
        }
    }
}
