use crate::data_struct::dawg::CompletionDawg;
use crate::data_struct::dawg::CompletionDawgSuspended;
use crate::data_struct::dawg::DawgList;
use crate::data_struct::dawg::DawgListSuspended;
use crate::data_struct::dawg::HH;
use crate::data_struct::gram_pairs::GramPairs;
use crate::data_struct::gram_pairs::GramPairsSuspended;
use crate::data_struct::letter_pairs::LetterPairs;
use crate::data_struct::letter_pairs::LetterPairsSuspended;
use crate::data_struct::letters::Letters;
use crate::data_struct::letters::LettersSuspended;
use crate::data_struct::string_list::StringArrayRef;
use crate::data_struct::string_list::StringArraySuspended;
use crate::model::dictionary::data_format::binpack::Revive;
use crate::model::dictionary::data_format::binpack::Suspend;
use crate::model::dictionary::gram_sets::GramSets;
use crate::model::dictionary::gram_sets::GramSetsSuspended;
use crate::model::dictionary::meta::DictionaryMeta;
use crate::model::dictionary::meta::DictionaryMetaSuspended;
use crate::model::dictionary::ParadigmListSuspended;
use crate::model::dictionary::PredictionSuffixValue;
use crate::model::dictionary::WordValue;
use crate::model::gram::GramRegistry;
use crate::model::gram::GramRegistrySuspended;
use crate::model::tag::TagRegistry;
use crate::model::tag::TagRegistrySuspended;
use crate::model::ParadigmList;

#[derive(Debug, Clone)]
pub struct DictionaryView<'d> {
    pub meta: DictionaryMeta<'d>,
    pub words: CompletionDawg<'d, WordValue>,
    pub p_t_given_w: Option<CompletionDawg<'d, HH>>,
    pub prediction_suffixes_dawgs: DawgList<'d, PredictionSuffixValue>,
    pub suffixes: StringArrayRef<'d>,
    pub prefixes: StringArrayRef<'d>,
    pub paradigms: ParadigmList<'d>,
    pub initial_letters: Letters<'d>,
    pub char_substitutes: LetterPairs<'d>,
    pub rare_cases: GramPairs<'d>,
    pub gram_sets: GramSets<'d>,
    pub grammemes: GramRegistry<'d>,
    pub gramtab: TagRegistry<'d>,
}

#[derive(Debug)]
pub struct DictionaryViewSuspended {
    pub(super) meta: DictionaryMetaSuspended,
    pub(super) words: CompletionDawgSuspended<WordValue>,
    pub(super) p_t_given_w: Option<CompletionDawgSuspended<HH>>,
    pub(super) prediction_suffixes_dawgs: DawgListSuspended<PredictionSuffixValue>,
    pub(super) suffixes: StringArraySuspended,
    pub(super) prefixes: StringArraySuspended,
    pub(super) paradigms: ParadigmListSuspended,
    pub(super) initial_letters: LettersSuspended,
    pub(super) char_substitutes: LetterPairsSuspended,
    pub(super) rare_cases: GramPairsSuspended,
    pub(super) gram_sets: GramSetsSuspended,
    pub(super) grammemes: GramRegistrySuspended,
    pub(super) gramtab: TagRegistrySuspended,
}

impl Suspend for DictionaryView<'_> {
    type Suspended = DictionaryViewSuspended;

    fn suspend(&self) -> DictionaryViewSuspended {
        DictionaryViewSuspended {
            meta: self.meta.suspend(),
            words: self.words.suspend(),
            p_t_given_w: self.p_t_given_w.as_ref().map(|d| d.suspend()),
            prediction_suffixes_dawgs: self.prediction_suffixes_dawgs.suspend(),
            suffixes: self.suffixes.suspend(),
            prefixes: self.prefixes.suspend(),
            paradigms: self.paradigms.suspend(),
            initial_letters: self.initial_letters.suspend(),
            char_substitutes: self.char_substitutes.suspend(),
            rare_cases: self.rare_cases.suspend(),
            gram_sets: self.gram_sets.suspend(),
            grammemes: self.grammemes.suspend(),
            gramtab: self.gramtab.suspend(),
        }
    }
}

impl Revive for DictionaryViewSuspended {
    type Revived<'d> = DictionaryView<'d>;

    unsafe fn revive<'d>(&self, source_bin: &'d [u8]) -> DictionaryView<'d> {
        DictionaryView {
            meta: self.meta.revive(source_bin),
            words: self.words.revive(source_bin),
            p_t_given_w: self.p_t_given_w.as_ref().map(|d| d.revive(source_bin)),
            prediction_suffixes_dawgs: self.prediction_suffixes_dawgs.revive(source_bin),
            suffixes: self.suffixes.revive(source_bin),
            prefixes: self.prefixes.revive(source_bin),
            paradigms: self.paradigms.revive(source_bin),
            initial_letters: self.initial_letters.revive(source_bin),
            char_substitutes: self.char_substitutes.revive(source_bin),
            rare_cases: self.rare_cases.revive(source_bin),
            gram_sets: self.gram_sets.revive(source_bin),
            grammemes: self.grammemes.revive(source_bin),
            gramtab: self.gramtab.revive(source_bin),
        }
    }
}

impl<'d> DictionaryView<'d> {
    pub fn new(
        meta: DictionaryMeta<'d>,
        words: CompletionDawg<'d, WordValue>,
        p_t_given_w: Option<CompletionDawg<'d, HH>>,
        prediction_suffixes_dawgs: DawgList<'d, PredictionSuffixValue>,
        suffixes: StringArrayRef<'d>,
        prefixes: StringArrayRef<'d>,
        paradigms: ParadigmList<'d>,
        initial_letters: Letters<'d>,
        char_substitutes: LetterPairs<'d>,
        rare_cases: GramPairs<'d>,
        gram_sets: GramSets<'d>,
        grammemes: GramRegistry<'d>,
        gramtab: TagRegistry<'d>,
    ) -> Self {
        DictionaryView {
            meta,
            words,
            p_t_given_w,
            prediction_suffixes_dawgs,
            suffixes,
            prefixes,
            paradigms,
            initial_letters,
            char_substitutes,
            rare_cases,
            gram_sets,
            grammemes,
            gramtab,
        }
    }
}
