use std::cmp::min;
use std::fmt;

use derive_more::From;
use lib::collections::dawg::CompletionDawg;
use lib::collections::dawg::HH;
use lib::collections::dawg_value::PredictionSuffixValue;
use lib::collections::dawg_value::WordValue;
#[cfg(feature = "dict_repr-repacked")]
use lib::collections::dictionary::DictionaryPack;
#[cfg(feature = "dict_repr-classic")]
use lib::collections::dictionary::DictionaryStorage;
use lib::collections::dictionary_meta::DictionaryMeta;
use lib::collections::gram_pairs::GramPairs;
use lib::collections::gram_registry::GramRegistry;
use lib::collections::gram_sets::GramSets;
use lib::collections::gram_sets::NumeralGrammemes;
use lib::collections::letter_pairs::LetterPairs;
use lib::collections::letters::Letters;
use lib::collections::paradigm::ParadigmEntry;
use lib::collections::paradigm::ParadigmId;
use lib::collections::paradigm::ParadigmIndex;
use lib::collections::paradigm::ParadigmList;
use lib::collections::str_stack::StrListRef;
use lib::collections::tag_registry::TagRegistry;
use lib::types::gram::GramSet;
use lib::types::tag::TagReg;

pub type PredictionSuffixesDawg<'d> = CompletionDawg<'d, PredictionSuffixValue>;

/// Open Corpora dictionary wrapper class.
#[derive(Debug, From)]
pub enum Dictionary {
    #[cfg(feature = "dict_repr-classic")]
    Classic(DictionaryStorage),
    #[cfg(feature = "dict_repr-repacked")]
    Repacked(DictionaryPack),
}

impl Dictionary {
    #[inline(always)]
    pub fn grammemes(&self) -> GramRegistry<'_> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.grammemes(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.grammemes(),
        }
    }

    #[inline(always)]
    pub fn gramtab(&self) -> TagRegistry<'_> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.gramtab(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.gramtab(),
        }
    }

    #[inline(always)]
    fn suffixes(&self) -> StrListRef<'_> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.suffixes(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.suffixes(),
        }
    }

    #[inline(always)]
    pub fn paradigms(&self) -> ParadigmList<'_> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.paradigms(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.paradigms(),
        }
    }

    #[inline(always)]
    pub fn words(&self) -> CompletionDawg<'_, WordValue> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.words(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.words(),
        }
    }

    #[inline(always)]
    pub fn p_t_given_w(&self) -> Option<CompletionDawg<HH>> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.p_t_given_w(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.p_t_given_w(),
        }
    }

    #[inline(always)]
    pub fn prediction_suffixes_dawg(&self, n: usize) -> CompletionDawg<PredictionSuffixValue> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.prediction_suffixes_dawg(n),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.prediction_suffixes_dawg(n),
        }
    }

    #[inline(always)]
    pub fn paradigm_prefixes(&self) -> StrListRef {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.paradigm_prefixes(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.paradigm_prefixes(),
        }
    }

    #[inline(always)]
    fn meta(&self) -> DictionaryMeta<'_> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.meta(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.meta(),
        }
    }

    #[inline(always)]
    pub fn initial_letters(&self) -> Letters<'_> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.initial_letters(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.initial_letters(),
        }
    }

    #[inline(always)]
    pub fn char_substitutes(&self) -> LetterPairs<'_> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.char_substitutes(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.char_substitutes(),
        }
    }

    #[inline(always)]
    pub fn rare_cases(&self) -> GramPairs<'_> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.rare_cases(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.rare_cases(),
        }
    }

    #[inline(always)]
    pub fn gram_sets(&self) -> GramSets<'_> {
        match self {
            #[cfg(feature = "dict_repr-classic")]
            Dictionary::Classic(inner) => inner.gram_sets(),
            #[cfg(feature = "dict_repr-repacked")]
            Dictionary::Repacked(inner) => inner.gram_sets(),
        }
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
        let para = self.paradigms().get(id).expect("paradigm");
        self.gramtab().get(para[idx.index()].tag_id()).unwrap()
    }

    #[inline]
    pub fn iter_paradigm_info<'d: 'i, 'i>(
        &'d self,
        id: ParadigmId,
    ) -> impl Iterator<Item = (&'d str, &'d TagReg, &'d str)> + 'i {
        self.paradigms()
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
        let paradigm = self.paradigms().get(id).expect("paradigm");
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
            let para = self.paradigms().get(id).expect("paradigm");
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
        self.rare_cases()
            .iter()
            .filter(|&(from, _to)| grammemes.contains(from))
            .fold(grammemes, |set, (from, to)| {
                (set - from.as_set()) | to.as_set()
            })
    }

    #[inline(always)]
    pub fn is_productive(&self, grammemes: GramSet) -> bool {
        (grammemes & self.gram_sets().non_productive()).is_empty()
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
        } = *self.gram_sets().numeral();

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
