use std::marker::PhantomData;

use byteorder::ByteOrder;
use smallvec::SmallVec;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binsegments::SegmentsBuilder;
use crate::binsegments::SegmentsView;
use crate::collections::dawg::DawgList;
use crate::collections::dawg::DawgValue;
use crate::data_format::binpack::adapter::dawg::CompletionDawgAdapter;

pub struct DawgListAdapter<V: DawgValue>(PhantomData<V>);

impl<V: DawgValue> MemView for DawgListAdapter<V> {
    type View<'d> = DawgList<'d, V>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let segments = SegmentsView::new(bin);

        let mut list = SmallVec::new();

        let num = segments.segments_number();
        for index in 0..num {
            let data = segments.get(index).unwrap();
            list.push(CompletionDawgAdapter::view(data));
        }

        DawgList::new(list)
    }
}

impl<V: DawgValue> MemViewPack for DawgListAdapter<V> {
    type View<'d> = DawgList<'d, V>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let mut segments = SegmentsBuilder::new();

        for dawg in view.iter() {
            segments.new_segment_from_slice(&CompletionDawgAdapter::pack::<Order>(dawg));
        }

        segments.build::<Order>()
    }
}
