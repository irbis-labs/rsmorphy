use smallvec::SmallVec;

use crate::data_struct::dawg::CompletionDawg;
use crate::data_struct::dawg::DawgDictionary;
use crate::data_struct::dawg::DawgGuide;
use crate::data_struct::dawg::DawgList;
use crate::data_struct::dawg::DictEntry;
use crate::data_struct::dawg::GuideEntry;
use crate::data_struct::dawg::HH;
use crate::data_struct::gram_pairs::GramPairs;
use crate::data_struct::letter_pairs::LetterPairs;
use crate::data_struct::letters::Letters;
use crate::data_struct::string_list::StringArray;
use crate::data_struct::string_list::StringArrayRef;
use crate::model::dictionary::gram_sets::GramSets;
use crate::model::dictionary::gram_sets::GramSetsOwned;
use crate::model::dictionary::meta::DictionaryMeta;
use crate::model::dictionary::meta::DictionaryMetaOwned;
use crate::model::dictionary::value::WordValue;
use crate::model::dictionary::ParadigmEntry;
use crate::model::dictionary::PredictionSuffixValue;
use crate::model::gram::Gram;
use crate::model::gram::GramRegistry;
use crate::model::gram::GramRegistryStorage;
use crate::model::tag::TagReg;
use crate::model::tag::TagRegistry;
use crate::model::ParadigmList;

/// Open Corpora dictionary wrapper class.
#[derive(Debug)]
pub struct DictionaryStruct {
    meta: DictionaryMetaOwned,
    words: (Box<[DictEntry]>, Box<[GuideEntry]>),
    p_t_given_w: Option<(Box<[DictEntry]>, Box<[GuideEntry]>)>,
    prediction_suffixes_dawgs: Vec<(Box<[DictEntry]>, Box<[GuideEntry]>)>,
    suffixes: StringArray,
    paradigms: (Box<[u32]>, Box<[ParadigmEntry]>),
    paradigm_prefixes: StringArray,
    initial_letters: SmallVec<[char; 32]>,
    char_substitutes: (SmallVec<[char; 4]>, SmallVec<[char; 4]>),
    rare_cases: SmallVec<[(Gram, Gram); 8]>,
    gram_sets: GramSetsOwned,
    grammemes: GramRegistryStorage,
    gramtab: Box<[TagReg]>,
}

impl DictionaryStruct {
    #[inline(always)]
    pub(super) fn new(
        meta: DictionaryMetaOwned,
        grammemes: GramRegistryStorage,
        gramtab: Box<[TagReg]>,
        suffixes: StringArray,
        paradigms: (Box<[u32]>, Box<[ParadigmEntry]>),
        words: (Box<[DictEntry]>, Box<[GuideEntry]>),
        p_t_given_w: Option<(Box<[DictEntry]>, Box<[GuideEntry]>)>,
        prediction_suffixes_dawgs: Vec<(Box<[DictEntry]>, Box<[GuideEntry]>)>,
        paradigm_prefixes: StringArray,
        initial_letters: SmallVec<[char; 32]>,
        char_substitutes: (SmallVec<[char; 4]>, SmallVec<[char; 4]>),
        rare_cases: SmallVec<[(Gram, Gram); 8]>,
        gram_sets: GramSetsOwned,
    ) -> Self {
        DictionaryStruct {
            meta,
            grammemes,
            gramtab,
            suffixes,
            paradigms,
            words,
            p_t_given_w,
            prediction_suffixes_dawgs,
            paradigm_prefixes,
            initial_letters,
            char_substitutes,
            rare_cases,
            gram_sets,
        }
    }

    #[inline(always)]
    pub fn grammemes(&self) -> GramRegistry<'_> {
        self.grammemes.to_ref()
    }

    #[inline(always)]
    pub fn gramtab(&self) -> TagRegistry<'_> {
        TagRegistry::new(&self.gramtab)
    }

    #[inline(always)]
    pub fn suffixes(&self) -> StringArrayRef<'_> {
        self.suffixes.as_ref()
    }

    #[inline(always)]
    pub fn paradigms(&self) -> ParadigmList<'_> {
        ParadigmList::new(&self.paradigms.0, &self.paradigms.1)
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
    pub fn prediction_suffixes_dawgs(&self) -> DawgList<PredictionSuffixValue> {
        let list = self
            .prediction_suffixes_dawgs
            .iter()
            .map(|(dict, guide)| {
                CompletionDawg::new(DawgDictionary::new(dict), DawgGuide::new(guide))
            })
            .collect();
        DawgList::new(list)
    }

    #[inline(always)]
    pub fn prediction_suffixes_dawg(&self, n: usize) -> CompletionDawg<PredictionSuffixValue> {
        let (dict, guide) = &self.prediction_suffixes_dawgs[n];
        CompletionDawg::new(DawgDictionary::new(dict), DawgGuide::new(guide))
    }

    #[inline(always)]
    pub fn gram_sets(&self) -> GramSets<'_> {
        self.gram_sets.get_view()
    }

    #[inline(always)]
    pub fn paradigm_prefixes(&self) -> StringArrayRef<'_> {
        self.paradigm_prefixes.as_ref()
    }

    #[inline(always)]
    pub fn meta(&self) -> DictionaryMeta<'_> {
        self.meta.get_view()
    }

    #[inline(always)]
    pub fn initial_letters(&self) -> Letters<'_> {
        Letters::new(&self.initial_letters)
    }

    #[inline(always)]
    pub fn char_substitutes(&self) -> LetterPairs<'_> {
        LetterPairs::new(&self.char_substitutes.0, &self.char_substitutes.1)
    }

    #[inline(always)]
    pub fn rare_cases(&self) -> GramPairs<'_> {
        GramPairs::new(&self.rare_cases)
    }
}
