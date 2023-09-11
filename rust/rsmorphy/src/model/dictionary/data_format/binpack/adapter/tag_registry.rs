use byteorder::ByteOrder;

use crate::model::dictionary::data_format::binpack::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::segments::SegmentsBuilder;
use crate::model::dictionary::data_format::binpack::segments::SegmentsView;
use crate::model::dictionary::data_format::binpack::util::convert_slice;
use crate::model::dictionary::data_format::binpack::MemView;
use crate::model::dictionary::data_format::binpack::MemViewPack;
use crate::model::tag::TagReg;
use crate::model::tag::TagRegistry;

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
