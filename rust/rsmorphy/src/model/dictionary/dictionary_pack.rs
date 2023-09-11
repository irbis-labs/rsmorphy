use smallvec::SmallVec;

use crate::data_struct::dawg::CompletionDawg;
use crate::data_struct::dawg::DawgList;
use crate::data_struct::dawg::DictEntry;
use crate::data_struct::dawg::GuideEntry;
use crate::data_struct::dawg::HH;
use crate::data_struct::gram_pairs::GramPairs;
use crate::data_struct::letter_pairs::LetterPairs;
use crate::data_struct::letters::Letters;
use crate::data_struct::string_list::StringArray;
use crate::data_struct::string_list::StringArrayRef;
use crate::model::dictionary::data_format::binpack::mem_pack;
use crate::model::dictionary::data_format::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::Revive;
use crate::model::dictionary::data_format::binpack::Suspend;
use crate::model::dictionary::dictionary_struct::DictionaryStruct;
use crate::model::dictionary::dictionary_view::DictionaryView;
use crate::model::dictionary::dictionary_view::DictionaryViewSuspended;
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
pub struct DictionaryPack {
    bin: BinPack,
    suspended: DictionaryViewSuspended,
}

unsafe impl Send for DictionaryPack {}
unsafe impl Sync for DictionaryPack {}

impl DictionaryPack {
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
        let bin = {
            let dict = DictionaryStruct::new(
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
            );
            let dict_view = DictionaryView::new(
                dict.meta(),
                dict.words(),
                dict.p_t_given_w(),
                dict.prediction_suffixes_dawgs(),
                dict.suffixes(),
                dict.paradigm_prefixes(),
                dict.paradigms(),
                dict.initial_letters(),
                dict.char_substitutes(),
                dict.rare_cases(),
                dict.gram_sets(),
                dict.grammemes(),
                dict.gramtab(),
            );
            mem_pack::pack::<byteorder::NativeEndian>(dict_view)
        };

        let suspended = mem_pack::view(&bin).suspend();

        DictionaryPack { bin, suspended }
    }

    #[inline(always)]
    pub fn words(&self) -> CompletionDawg<'_, WordValue> {
        unsafe { self.suspended.words.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn p_t_given_w(&self) -> Option<CompletionDawg<'_, HH>> {
        self.suspended
            .p_t_given_w
            .as_ref()
            .map(|dawg| unsafe { dawg.revive(&self.bin) })
    }

    #[inline(always)]
    pub fn prediction_suffixes_dawgs(&self) -> DawgList<PredictionSuffixValue> {
        unsafe { self.suspended.prediction_suffixes_dawgs.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn prediction_suffixes_dawg(&self, n: usize) -> CompletionDawg<PredictionSuffixValue> {
        unsafe { self.suspended.prediction_suffixes_dawgs[n].revive(&self.bin) }
    }

    #[inline(always)]
    pub fn suffixes(&self) -> StringArrayRef<'_> {
        unsafe { self.suspended.suffixes.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn paradigms(&self) -> ParadigmList<'_> {
        unsafe { self.suspended.paradigms.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn para(&self) -> ParadigmList<'_> {
        self.paradigms()
    }

    #[inline(always)]
    pub fn paradigm_prefixes(&self) -> StringArrayRef {
        unsafe { self.suspended.prefixes.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn initial_letters(&self) -> Letters<'_> {
        unsafe { self.suspended.initial_letters.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn char_substitutes(&self) -> LetterPairs<'_> {
        unsafe { self.suspended.char_substitutes.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn rare_cases(&self) -> GramPairs<'_> {
        unsafe { self.suspended.rare_cases.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn gram_sets(&self) -> GramSets<'_> {
        unsafe { self.suspended.gram_sets.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn meta(&self) -> DictionaryMeta<'_> {
        unsafe { self.suspended.meta.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn grammemes(&self) -> GramRegistry<'_> {
        unsafe { self.suspended.grammemes.revive(&self.bin) }
    }

    #[inline(always)]
    pub fn gramtab(&self) -> TagRegistry<'_> {
        unsafe { self.suspended.gramtab.revive(&self.bin) }
    }
}
