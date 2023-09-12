use byteorder::ByteOrder;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binsegments::SegmentsBuilder;
use crate::binsegments::SegmentsView;
use crate::binutils::convert_slice;
use crate::collections::paradigm::ParadigmEntry;
use crate::collections::paradigm::ParadigmList;

pub struct ParadigmListAdapter;

#[repr(usize)]
enum Segment {
    Idx  = 0,
    Data = 1,
}

impl MemView for ParadigmListAdapter {
    type View<'d> = ParadigmList<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let segments = SegmentsView::new(bin);

        let idx = {
            let segment = segments.get(Segment::Idx as usize).unwrap();
            unsafe { convert_slice(segment) }.unwrap()
        };

        let data = {
            let segment = segments.get(Segment::Data as usize).unwrap();
            unsafe { convert_slice(segment) }.unwrap()
        };

        ParadigmList::new(idx, data)
    }
}

impl MemViewPack for ParadigmListAdapter {
    type View<'d> = ParadigmList<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let mut segments = SegmentsBuilder::new();

        let (idx, data) = (*view).into_inner();

        {
            let bytes: Box<[u8]> = idx
                .iter()
                .copied()
                .flat_map(|x| {
                    let mut buf = [0u8; 4];
                    Order::write_u32(&mut buf, x);
                    buf.into_iter()
                })
                .collect();
            let num = segments.new_segment_from_slice(&bytes);
            assert_eq!(num, Segment::Idx as usize);
        }

        {
            let bytes: Box<[u8]> = data
                .iter()
                .copied()
                .flat_map(
                    |ParadigmEntry {
                         prefix_id,
                         tag_id,
                         suffix_id,
                     }| {
                        let mut buf = [[0u8; 2]; 3];
                        Order::write_u16(&mut buf[0], prefix_id);
                        Order::write_u16(&mut buf[1], tag_id);
                        Order::write_u16(&mut buf[2], suffix_id);
                        buf.into_iter().flat_map(|x| x.into_iter())
                    },
                )
                .collect();
            let num = segments.new_segment_from_slice(&bytes);
            assert_eq!(num, Segment::Data as usize);
        }

        segments.build::<Order>()
    }
}
