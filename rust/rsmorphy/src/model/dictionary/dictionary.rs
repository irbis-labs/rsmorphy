use std::collections::HashMap;
use std::fmt;

use inlinable_string::InlinableString;
use smallvec::SmallVec;
use string_cache::DefaultAtom;

use crate::dawg::CompletionDawg;
pub use crate::dawg::HH;
pub use crate::dawg::HHH;
use crate::model::dictionary::numeral_agreement::NumeralGrammemes;
use crate::model::dictionary::value::WordValue;
use crate::model::dictionary::ParadigmEntry;
use crate::model::dictionary::PredictionSuffixValue;
use crate::model::gram::Gram;
use crate::model::gram::GramRegistry;
use crate::model::gram::GramSet;
use crate::model::tag::TagReg;
use crate::model::ParadigmId;
use crate::model::ParadigmIndex;
use crate::model::Paradigms;

pub type PredictionSuffixesDawg = CompletionDawg<PredictionSuffixValue>;

/// Open Corpora dictionary wrapper class.
#[derive(Debug)]
pub struct Dictionary {
    pub grammemes: GramRegistry,
    pub gramtab: Vec<TagReg>,
    pub suffixes: Vec<DefaultAtom>,
    pub paradigms: Paradigms,
    pub words: CompletionDawg<WordValue>,
    pub p_t_given_w: Option<CompletionDawg<HH>>,
    pub prediction_suffixes_dawgs: Vec<PredictionSuffixesDawg>,
    pub paradigm_prefixes: Vec<DefaultAtom>,
    pub paradigm_prefixes_rev: Vec<(u16, DefaultAtom)>,
    pub max_suffix_length: usize,
    pub char_substitutes: HashMap<InlinableString, InlinableString>,
    pub rare_cases: SmallVec<[(Gram, Gram); 8]>,
    pub non_productive: GramSet,
    pub numeral_grammemes: NumeralGrammemes,
}

impl Dictionary {
    pub fn prediction_splits(&self, word_len: usize) -> impl Iterator<Item = usize> {
        (1..=self.max_suffix_length.min(word_len - 1)).rev()
    }

    #[inline]
    pub fn para(&self) -> &Paradigms {
        &self.paradigms
    }

    /// Return tag as a string
    #[inline]
    pub fn get_tag(&self, id: ParadigmId, idx: ParadigmIndex) -> &TagReg {
        let para = self.para().get(id).expect("paradigm");
        &self.gramtab[para[idx.index()].tag_id() as usize]
    }

    #[inline]
    pub fn iter_paradigm_info<'a: 'i, 'i>(
        &'a self,
        id: ParadigmId,
    ) -> impl Iterator<Item = (&'a str, &'a TagReg, &'a str)> + 'i {
        self.para()
            .get(id)
            .expect("paradigm")
            .iter()
            .map(move |entry: &'a ParadigmEntry| self.paradigm_entry_info(*entry))
    }

    /// Return a tuple
    ///     (prefix, tag, suffix)
    /// tuples representing the paradigm entry.
    #[inline]
    fn paradigm_entry_info(&self, entry: ParadigmEntry) -> (&str, &TagReg, &str) {
        (
            &self.paradigm_prefixes[entry.prefix_id() as usize],
            &self.gramtab[entry.tag_id() as usize],
            &self.suffixes[entry.suffix_id() as usize],
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
                grammemes - self.grammemes.get_reg_by_gram(gram).incompatible
            })
    }

    pub fn fix_rare_cases(&self, mut grammemes: GramSet) -> GramSet {
        for &(from, to) in &self.rare_cases {
            if grammemes.contains(from) {
                grammemes.remove(from);
                grammemes.insert(to);
            }
        }
        grammemes
    }

    #[inline(always)]
    pub fn is_productive(&self, grammemes: GramSet) -> bool {
        (grammemes & self.non_productive).is_empty()
    }

    /// Returns grammemes that are required to be present in the word.
    /// The number should be passed similar to `(number.abs() % 100) as u8`.
    pub fn numeral_agreement_grammemes(&self, grammemes: GramSet, num: u8) -> GramSet {
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
        } = self.numeral_grammemes;

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
