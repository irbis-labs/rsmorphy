use std::marker::PhantomData;

use byteorder::ByteOrder;

use super::super::dawg::DawgDictionaryAdapter;
use super::super::dawg::DawgGuideAdapter;
use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binsegments::SegmentsBuilder;
use crate::binsegments::SegmentsView;
use crate::binutils::bytes_to_ref;
use crate::binutils::ref_to_bytes;
use crate::binutils::rewrite_u32;
use crate::collections::dawg::CompletionDawg;
use crate::collections::dawg::DawgValue;

#[repr(C, align(64))]
pub struct CompletionDawgMeta {
    root: u32,
    _reserved: [u8; 60],
}

#[repr(usize)]
enum Segment {
    Meta  = 0,
    Dict  = 1,
    Guide = 2,
}

pub struct CompletionDawgAdapter<V: DawgValue>(PhantomData<V>);

impl<V: DawgValue> MemView for CompletionDawgAdapter<V> {
    type View<'d> = CompletionDawg<'d, V>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let segments = SegmentsView::new(bin);

        let _meta = {
            let data = segments.get(Segment::Meta as usize).unwrap();
            unsafe { bytes_to_ref::<CompletionDawgMeta>(data) }
        };

        let dict = {
            let data = segments.get(Segment::Dict as usize).unwrap();
            DawgDictionaryAdapter::view(data)
        };

        let guide = {
            let data = segments.get(Segment::Guide as usize).unwrap();
            DawgGuideAdapter::view(data)
        };

        // Validate internal consistency of the data.
        {
            let (dict, _guide) = (dict.into_inner(), guide.into_inner());
            assert!(dict.len() > 0, "assert dict.len(): {} > 0", dict.len());
            assert!(
                _meta.root < dict.len() as u32,
                "assert root: {} < dict.len(): {}",
                _meta.root,
                dict.len()
            );
            // TODO validate units of dict and guide.
        }

        CompletionDawg::new(dict, guide)
    }
}

impl<V: DawgValue> MemViewPack for CompletionDawgAdapter<V> {
    type View<'d> = CompletionDawg<'d, V>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let mut segments = SegmentsBuilder::new();

        let (dict, guide) = (*view).into_inner();

        {
            let mut meta = CompletionDawgMeta {
                root: 0,
                _reserved: [0; 60],
            };
            rewrite_u32::<Order>(&mut meta.root);

            let data = unsafe { ref_to_bytes(&meta) };
            assert_eq!(data.len(), 64);

            let num = segments.new_segment_from_slice(data);
            assert_eq!(num, Segment::Meta as usize);
        }

        {
            let num = segments.new_segment_from_slice(&DawgDictionaryAdapter::pack::<Order>(&dict));
            assert_eq!(num, Segment::Dict as usize);
        }

        {
            let num = segments.new_segment_from_slice(&DawgGuideAdapter::pack::<Order>(&guide));
            assert_eq!(num, Segment::Guide as usize);
        }

        segments.build::<Order>()
    }
}
