use byteorder::ByteOrder;

use crate::data_struct::string_list::StringArrayRef;
use crate::model::dictionary::data_format::binpack::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::segments::SegmentsBuilder;
use crate::model::dictionary::data_format::binpack::segments::SegmentsView;
use crate::model::dictionary::data_format::binpack::util::convert_slice;
use crate::model::dictionary::data_format::binpack::MemView;
use crate::model::dictionary::data_format::binpack::MemViewPack;

pub struct StringArrayAdapter;

#[repr(usize)]
enum Segment {
    Ends = 0,
    Data = 1,
}

impl MemView for StringArrayAdapter {
    type View<'d> = StringArrayRef<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let segments = SegmentsView::new(bin);

        let ends = {
            let segment = segments.get(Segment::Ends as usize).unwrap();
            unsafe { convert_slice(segment) }.unwrap()
        };

        let data = {
            let segment = segments.get(Segment::Data as usize).unwrap();
            unsafe { convert_slice(segment) }.unwrap()
        };

        unsafe { StringArrayRef::new_unchecked(data, ends) }
    }
}

impl MemViewPack for StringArrayAdapter {
    type View<'d> = StringArrayRef<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let mut segments = SegmentsBuilder::new();

        let (data, ends) = (*view).into_inner();

        {
            let bytes: Box<[u8]> = ends
                .iter()
                .copied()
                .flat_map(|x| {
                    let mut buf = [0u8; 4];
                    Order::write_u32(&mut buf, x);
                    buf.into_iter()
                })
                .collect();
            let num = segments.new_segment_from_slice(&bytes);
            assert_eq!(num, Segment::Ends as usize);
        }

        {
            let num = segments.new_segment_from_slice(data);
            assert_eq!(num, Segment::Data as usize);
        }

        segments.build::<Order>()
    }
}
