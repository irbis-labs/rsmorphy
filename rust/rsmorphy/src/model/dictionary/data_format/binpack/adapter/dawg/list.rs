use std::marker::PhantomData;

use byteorder::ByteOrder;
use smallvec::SmallVec;

use crate::data_struct::dawg::DawgList;
use crate::data_struct::dawg::DawgValue;
use crate::model::dictionary::data_format::binpack::adapter::dawg::CompletionDawgAdapter;
use crate::model::dictionary::data_format::binpack::segments::SegmentsBuilder;
use crate::model::dictionary::data_format::binpack::segments::SegmentsView;
use crate::model::dictionary::data_format::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::MemView;
use crate::model::dictionary::data_format::binpack::MemViewPack;

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
