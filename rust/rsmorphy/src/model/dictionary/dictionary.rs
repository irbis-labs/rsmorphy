use std::cmp::min;
use std::fmt;

use crate::data_struct::dawg::CompletionDawg;
use crate::data_struct::dawg::HH;
use crate::data_struct::letter_pairs::LetterPairs;
use crate::data_struct::letters::Letters;
use crate::data_struct::string_list::StringArrayRef;
use crate::model::dictionary::gram_sets::NumeralGrammemes;
use crate::model::dictionary::meta::DictionaryMeta;
use crate::model::dictionary::value::WordValue;
use crate::model::dictionary::ParadigmEntry;
use crate::model::dictionary::PredictionSuffixValue;
use crate::model::gram::GramRegistry;
use crate::model::gram::GramSet;
use crate::model::tag::TagReg;
use crate::model::tag::TagRegistry;
use crate::model::ParadigmId;
use crate::model::ParadigmIndex;
use crate::model::ParadigmList;

pub type PredictionSuffixesDawg<'d> = CompletionDawg<'d, PredictionSuffixValue>;

// pub type DictionaryInner = super::dictionary_struct::DictionaryStruct;
pub type DictionaryInner = super::dictionary_pack::DictionaryPack;

/// Open Corpora dictionary wrapper class.
#[derive(Debug)]
pub struct Dictionary {
    inner: DictionaryInner,
}

impl Dictionary {
    #[inline(always)]
    pub(in crate::model::dictionary) fn new(inner: DictionaryInner) -> Self {
        Dictionary { inner }
    }

    #[inline(always)]
    pub fn grammemes(&self) -> GramRegistry<'_> {
        self.inner.grammemes()
    }

    #[inline(always)]
    pub fn gramtab(&self) -> TagRegistry<'_> {
        self.inner.gramtab()
    }

    #[inline(always)]
    fn suffixes(&self) -> StringArrayRef<'_> {
        self.inner.suffixes()
    }

    #[inline(always)]
    pub fn paradigms(&self) -> ParadigmList<'_> {
        self.inner.paradigms()
    }

    #[inline(always)]
    pub fn para(&self) -> ParadigmList<'_> {
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
    pub fn prediction_suffixes_dawg(&self, n: usize) -> CompletionDawg<PredictionSuffixValue> {
        self.inner.prediction_suffixes_dawg(n)
    }

    #[inline(always)]
    pub fn paradigm_prefixes(&self) -> StringArrayRef {
        self.inner.paradigm_prefixes()
    }

    #[inline(always)]
    fn meta(&self) -> DictionaryMeta<'_> {
        self.inner.meta()
    }

    #[inline(always)]
    pub fn initial_letters(&self) -> Letters<'_> {
        self.inner.initial_letters()
    }

    #[inline(always)]
    pub fn char_substitutes(&self) -> LetterPairs<'_> {
        self.inner.char_substitutes()
    }

    // ---------------------------------------------------------------------------------------------

    #[inline(always)]
    pub fn prediction_splits(&self, word_len: usize) -> impl Iterator<Item = usize> {
        let max_suffix_len = self.meta().max_suffix_length();
        // At least 1 char should be left for the stem.
        let max_word_len = word_len.saturating_sub(1);

        let max_suffix_len = min(max_suffix_len, max_word_len);

        // From the longest suffixes to the shortest.
        (1..=max_suffix_len).rev()
    }

    /// Return tag as a string
    #[inline]
    pub fn get_tag(&self, id: ParadigmId, idx: ParadigmIndex) -> &TagReg {
        let para = self.para().get(id).expect("paradigm");
        self.gramtab().get(para[idx.index()].tag_id()).unwrap()
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
            .map(move |entry: &'d ParadigmEntry| self.paradigm_entry_info(*entry))
    }

    /// Return a tuple
    ///     (prefix, tag, suffix)
    /// tuples representing the paradigm entry.
    #[inline]
    pub fn paradigm_entry_info(&self, entry: ParadigmEntry) -> (&str, &TagReg, &str) {
        (
            self.paradigm_prefixes()
                .get(entry.prefix_id() as usize)
                .unwrap(),
            self.gramtab().get(entry.tag_id()).unwrap(),
            self.suffixes().get(entry.suffix_id() as usize).unwrap(),
        )
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
        let (prefix, _, suffix) = self.paradigm_entry_info(para_entry);
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
            let (normal_prefix, _, normal_suffix) = self.paradigm_entry_info(para[0]);
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
                let gram_reg = self.grammemes().get_reg_by_gram(gram);
                let incompatible = gram_reg.map(|gr| gr.incompatible).unwrap_or_default();
                grammemes - incompatible
            })
    }

    pub fn fix_rare_cases(&self, grammemes: GramSet) -> GramSet {
        self.inner
            .rare_cases()
            .iter()
            .filter(|&(from, _to)| grammemes.contains(from))
            .fold(grammemes, |set, (from, to)| {
                (set - from.as_set()) | to.as_set()
            })
    }

    #[inline(always)]
    pub fn is_productive(&self, grammemes: GramSet) -> bool {
        (grammemes & self.inner.gram_sets().non_productive()).is_empty()
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
        } = *self.inner.gram_sets().numeral();

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
