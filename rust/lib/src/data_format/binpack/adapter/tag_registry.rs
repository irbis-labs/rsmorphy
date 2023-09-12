use byteorder::ByteOrder;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binsegments::SegmentsBuilder;
use crate::binsegments::SegmentsView;
use crate::binutils::convert_slice;
use crate::collections::tag_registry::TagRegistry;
use crate::types::tag::TagReg;

pub struct TagRegistryAdapter;

#[repr(usize)]
enum Segment {
    TagReg = 0,
}

impl MemView for TagRegistryAdapter {
    type View<'d> = TagRegistry<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let segments = SegmentsView::new(bin);

        let tags = {
            let segment = segments.get(Segment::TagReg as usize).unwrap();
            unsafe { convert_slice(segment) }.unwrap()
        };

        TagRegistry::new(tags)
    }
}

impl MemViewPack for TagRegistryAdapter {
    type View<'d> = TagRegistry<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let mut segments = SegmentsBuilder::new();

        let tags = (*view).into_inner();

        {
            let reg_map: Box<[TagReg]> = tags
                .iter()
                .copied()
                .map(|reg| reg.into_order::<Order>())
                .collect();
            let num = segments.new_segment_from_slice(unsafe { convert_slice(&reg_map) }.unwrap());
            assert_eq!(num, Segment::TagReg as usize);
        }

        segments.build::<Order>()
    }
}
