use byteorder::ByteOrder;

use crate::model::dictionary::data_format::binpack::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::segments::SegmentsBuilder;
use crate::model::dictionary::data_format::binpack::segments::SegmentsView;
use crate::model::dictionary::data_format::binpack::util::convert_slice;
use crate::model::dictionary::data_format::binpack::MemView;
use crate::model::dictionary::data_format::binpack::MemViewPack;
use crate::model::gram::GramRegistry;
use crate::model::gram::LatMapEntry;
use crate::GramReg;

pub struct GramRegistryAdapter;

#[repr(usize)]
enum Segment {
    RegMap = 0,
    LatMap = 1,
}

impl MemView for GramRegistryAdapter {
    type View<'d> = GramRegistry<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let segments = SegmentsView::new(bin);

        let reg_map = {
            let segment = segments.get(Segment::RegMap as usize).unwrap();
            unsafe { convert_slice(segment) }.unwrap()
        };

        let lat_map = {
            let segment = segments.get(Segment::LatMap as usize).unwrap();
            unsafe { convert_slice(segment) }.unwrap()
        };

        unsafe { GramRegistry::new(reg_map, lat_map) }
    }
}

impl MemViewPack for GramRegistryAdapter {
    type View<'d> = GramRegistry<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let mut segments = SegmentsBuilder::new();

        let (reg_map, lat_map) = (*view).into_inner();

        {
            let reg_map: Box<[GramReg]> = reg_map
                .iter()
                .copied()
                .map(|reg| reg.into_order::<Order>())
                .collect();
            let num = segments.new_segment_from_slice(unsafe { convert_slice(&reg_map) }.unwrap());
            assert_eq!(num, Segment::RegMap as usize);
        }

        {
            let lat_map: Box<[LatMapEntry]> = lat_map
                .iter()
                .copied()
                .map(|lm_entry| lm_entry.into_order::<Order>())
                .collect();
            let num = segments.new_segment_from_slice(unsafe { convert_slice(&lat_map) }.unwrap());
            assert_eq!(num, Segment::LatMap as usize);
        }

        segments.build::<Order>()
    }
}
