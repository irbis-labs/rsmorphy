use std::borrow::Cow;

use crate::binpack::BinPackRef;
use crate::collections::dawg::CompletionDawg;
use crate::collections::dawg::DawgList;
use crate::collections::dawg::HH;
use crate::collections::dawg_value::PredictionSuffixValue;
use crate::collections::dawg_value::WordValue;
use crate::collections::dictionary::DictionaryStorage;
use crate::collections::dictionary::DictionaryView;
use crate::collections::dictionary::DictionaryViewSuspended;
use crate::collections::dictionary_meta::DictionaryMeta;
use crate::collections::gram_pairs::GramPairs;
use crate::collections::gram_registry::GramRegistry;
use crate::collections::gram_sets::GramSets;
use crate::collections::letter_pairs::LetterPairs;
use crate::collections::letters::Letters;
use crate::collections::paradigm::ParadigmList;
use crate::collections::string_list::StringArrayRef;
use crate::collections::tag_registry::TagRegistry;
use crate::cryostasis::Revive;
use crate::cryostasis::Suspend;
use crate::data_format::binpack::mem_pack;

/// Open Corpora dictionary wrapper class.
#[derive(Debug)]
pub struct DictionaryPack {
    bin: Cow<'static, BinPackRef>,
    suspended: DictionaryViewSuspended,
}

unsafe impl Send for DictionaryPack {}
unsafe impl Sync for DictionaryPack {}

impl DictionaryPack {
    #[inline(always)]
    pub fn from_storage(dict: &DictionaryStorage) -> Self {
        let dict_view = DictionaryView::from_storage(&dict);
        let bin = mem_pack::pack::<byteorder::NativeEndian>(dict_view);
        DictionaryPack::from_bin_pack(Cow::Owned(bin))
    }

    #[inline(always)]
    pub fn from_bin_pack(bin: Cow<'static, BinPackRef>) -> Self {
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
