use byteorder::ByteOrder;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binsegments::SegmentsBuilder;
use crate::binsegments::SegmentsView;
use crate::binutils::convert_slice;
use crate::collections::gram_registry::GramRegistry;
use crate::collections::gram_registry::LatMapEntry;
use crate::types::gram::GramReg;

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

        GramRegistry::new(reg_map, lat_map)
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
