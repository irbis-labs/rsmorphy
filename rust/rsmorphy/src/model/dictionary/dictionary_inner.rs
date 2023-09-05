use std::collections::HashMap;

use inlinable_string::InlinableString;
use smallvec::SmallVec;
use string_cache::DefaultAtom;

use crate::dawg::CompletionDawg;
use crate::dawg::DawgDictionary;
use crate::dawg::DawgGuide;
use crate::dawg::DictEntry;
use crate::dawg::GuideEntry;
use crate::dawg::HH;
use crate::model::dictionary::numeral_agreement::NumeralGrammemes;
use crate::model::dictionary::value::WordValue;
use crate::model::dictionary::ParadigmEntry;
use crate::model::dictionary::PredictionSuffixValue;
use crate::model::gram::Gram;
use crate::model::gram::GramRegistry;
use crate::model::gram::GramRegistryStorage;
use crate::model::gram::GramSet;
use crate::model::tag::TagReg;
use crate::model::Paradigms;

/// Open Corpora dictionary wrapper class.
#[derive(Debug)]
pub struct DictionaryInner {
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
}

impl DictionaryInner {
    #[inline(always)]
    pub(super) fn new(
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
        DictionaryInner {
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
        }
    }

    #[inline(always)]
    pub fn grammemes(&self) -> GramRegistry<'_> {
        self.grammemes.to_ref()
    }

    #[inline(always)]
    pub fn gramtab(&self) -> &[TagReg] {
        &self.gramtab
    }

    #[inline(always)]
    pub fn suffixes(&self) -> &[DefaultAtom] {
        &self.suffixes
    }

    #[inline(always)]
    pub fn paradigms(&self) -> Paradigms<'_> {
        Paradigms::new(&self.paradigms.0, &self.paradigms.1)
    }

    #[inline(always)]
    pub fn para(&self) -> Paradigms<'_> {
        Paradigms::new(&self.paradigms.0, &self.paradigms.1)
    }

    #[inline(always)]
    pub fn words(&self) -> CompletionDawg<'_, WordValue> {
        let (dict, guide) = &self.words;
        CompletionDawg::new(DawgDictionary::new(dict), DawgGuide::new(guide))
    }

    #[inline(always)]
    pub fn p_t_given_w(&self) -> Option<CompletionDawg<HH>> {
        let (dict, guide) = self.p_t_given_w.as_ref()?;
        Some(CompletionDawg::new(
            DawgDictionary::new(dict),
            DawgGuide::new(guide),
        ))
    }

    #[inline(always)]
    pub fn prediction_suffixes_dawgs(&self, n: usize) -> CompletionDawg<PredictionSuffixValue> {
        let (dict, guide) = &self.prediction_suffixes_dawgs[n];
        CompletionDawg::new(DawgDictionary::new(dict), DawgGuide::new(guide))
    }

    #[inline(always)]
    pub fn paradigm_prefixes_len(&self) -> usize {
        self.paradigm_prefixes.len()
    }

    #[inline(always)]
    pub fn iter_paradigm_prefixes(
        &self,
    ) -> impl Iterator<Item = &str> + DoubleEndedIterator + ExactSizeIterator + '_ {
        self.paradigm_prefixes.iter().map(|s| s.as_ref())
    }

    #[inline(always)]
    pub fn max_suffix_length(&self) -> usize {
        self.max_suffix_length as usize
    }

    #[inline(always)]
    pub fn char_substitutes(&self) -> &HashMap<InlinableString, InlinableString> {
        &self.char_substitutes
    }

    /// Return a tuple
    ///     (prefix, tag, suffix)
    /// tuples representing the paradigm entry.
    #[inline]
    pub fn paradigm_entry_info(&self, entry: ParadigmEntry) -> (&str, &TagReg, &str) {
        (
            &self.paradigm_prefixes[entry.prefix_id() as usize],
            &self.gramtab[entry.tag_id() as usize],
            &self.suffixes[entry.suffix_id() as usize],
        )
    }

    #[inline(always)]
    pub fn rare_cases(&self) -> &[(Gram, Gram)] {
        &self.rare_cases
    }

    #[inline(always)]
    pub fn non_productive(&self) -> GramSet {
        self.non_productive
    }

    #[inline(always)]
    pub fn numeral_grammemes(&self) -> &NumeralGrammemes {
        &self.numeral_grammemes
    }
}
