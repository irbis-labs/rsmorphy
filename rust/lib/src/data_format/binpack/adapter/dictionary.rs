use byteorder::ByteOrder;

use super::super::adapter::dawg::CompletionDawgAdapter;
use super::super::adapter::dawg::DawgListAdapter;
use super::super::adapter::letters::LettersAdapter;
use super::super::adapter::paradigm_list::ParadigmListAdapter;
use super::super::adapter::string_list::StringArrayAdapter;
use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binsegments::SegmentsBuilder;
use crate::binsegments::SegmentsView;
use crate::collections::dictionary::DictionaryView;
use crate::data_format::binpack::adapter::gram_pairs::GramPairsAdapter;
use crate::data_format::binpack::adapter::gram_registry::GramRegistryAdapter;
use crate::data_format::binpack::adapter::gram_sets::GramSetsAdapter;
use crate::data_format::binpack::adapter::letter_pairs::LetterPairsAdapter;
use crate::data_format::binpack::adapter::meta::DictionaryMetaAdapter;
use crate::data_format::binpack::adapter::tag_registry::TagRegistryAdapter;

pub struct DictionaryAdapter;

#[repr(usize)]
enum Segment {
    Meta                    = 0,
    Words                   = 1,
    PTWProb                 = 2,
    PredictionSuffixesDawgs = 3,
    Suffixes                = 4,
    ParadigmPrefixes        = 5,
    Paradigms               = 6,
    InitialLetters          = 7,
    CharSubstitutes         = 8,
    RareCases               = 9,
    GramSets                = 10,
    Grammemes               = 11,
    Gramtab                 = 12,
}

impl MemView for DictionaryAdapter {
    type View<'d> = DictionaryView<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let segments = SegmentsView::new(bin);

        let meta = {
            let data = segments.get(Segment::Meta as usize).unwrap();
            DictionaryMetaAdapter::view(data)
        };

        let words = {
            let data = segments.get(Segment::Words as usize).unwrap();
            CompletionDawgAdapter::view(data)
        };

        let p_t_given_w = {
            let data = segments.get(Segment::PTWProb as usize).unwrap();
            if data.len() == 0 {
                None
            } else {
                Some(CompletionDawgAdapter::view(data))
            }
        };

        let prediction_suffixes_dawgs = {
            let data = segments
                .get(Segment::PredictionSuffixesDawgs as usize)
                .unwrap();
            DawgListAdapter::view(data)
        };

        let suffixes = {
            let data = segments.get(Segment::Suffixes as usize).unwrap();
            StringArrayAdapter::view(data)
        };

        let prefixes = {
            let data = segments.get(Segment::ParadigmPrefixes as usize).unwrap();
            StringArrayAdapter::view(data)
        };

        let paradigms = {
            let data = segments.get(Segment::Paradigms as usize).unwrap();
            ParadigmListAdapter::view(data)
        };

        let initial_letters = {
            let data = segments.get(Segment::InitialLetters as usize).unwrap();
            LettersAdapter::view(data)
        };

        let char_substitutes = {
            let data = segments.get(Segment::CharSubstitutes as usize).unwrap();
            LetterPairsAdapter::view(data)
        };

        let rare_cases = {
            let data = segments.get(Segment::RareCases as usize).unwrap();
            GramPairsAdapter::view(data)
        };

        let gram_sets = {
            let data = segments.get(Segment::GramSets as usize).unwrap();
            GramSetsAdapter::view(data)
        };

        let grammemes = {
            let data = segments.get(Segment::Grammemes as usize).unwrap();
            GramRegistryAdapter::view(data)
        };

        let gramtab = {
            let data = segments.get(Segment::Gramtab as usize).unwrap();
            TagRegistryAdapter::view(data)
        };

        DictionaryView::new(
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
        )
    }
}

impl MemViewPack for DictionaryAdapter {
    type View<'d> = DictionaryView<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let mut segments = SegmentsBuilder::new();

        {
            let data = DictionaryMetaAdapter::pack::<Order>(&view.meta);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::Meta as usize);
        }

        {
            let data = CompletionDawgAdapter::pack::<Order>(&view.words);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::Words as usize);
        }

        {
            let num = match &view.p_t_given_w {
                Some(dawg) => {
                    let data = CompletionDawgAdapter::pack::<Order>(dawg);
                    segments.new_segment_from_slice(&data)
                }
                None => segments.new_segment_from_slice(&[]),
            };
            assert_eq!(num, Segment::PTWProb as usize);
        }

        {
            let data = DawgListAdapter::pack::<Order>(&view.prediction_suffixes_dawgs);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::PredictionSuffixesDawgs as usize);
        }

        {
            let data = StringArrayAdapter::pack::<Order>(&view.suffixes);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::Suffixes as usize);
        }

        {
            let data = StringArrayAdapter::pack::<Order>(&view.prefixes);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::ParadigmPrefixes as usize);
        }

        {
            let data = ParadigmListAdapter::pack::<Order>(&view.paradigms);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::Paradigms as usize);
        }

        {
            let data = LettersAdapter::pack::<Order>(&view.initial_letters);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::InitialLetters as usize);
        }

        {
            let data = LetterPairsAdapter::pack::<Order>(&view.char_substitutes);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::CharSubstitutes as usize);
        }

        {
            let data = GramPairsAdapter::pack::<Order>(&view.rare_cases);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::RareCases as usize);
        }

        {
            let data = GramSetsAdapter::pack::<Order>(&view.gram_sets);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::GramSets as usize);
        }

        {
            let data = GramRegistryAdapter::pack::<Order>(&view.grammemes);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::Grammemes as usize);
        }

        {
            let data = TagRegistryAdapter::pack::<Order>(&view.gramtab);
            let num = segments.new_segment_from_slice(&data);
            assert_eq!(num, Segment::Gramtab as usize);
        }

        segments.build::<Order>()
    }
}
