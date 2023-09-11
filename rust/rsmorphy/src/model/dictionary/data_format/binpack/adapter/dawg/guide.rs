use byteorder::ByteOrder;

use crate::data_struct::dawg::DawgGuide;
use crate::data_struct::dawg::GuideEntry;
use crate::model::dictionary::data_format::binpack::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::util::convert_slice;
use crate::model::dictionary::data_format::binpack::MemView;
use crate::model::dictionary::data_format::binpack::MemViewPack;

pub struct DawgGuideAdapter;

impl MemView for DawgGuideAdapter {
    type View<'d> = DawgGuide<'d>;

    fn view(data: &[u8]) -> Self::View<'_> {
        let units = unsafe { convert_slice(data) }.unwrap();
        DawgGuide::new(units)
    }
}

impl MemViewPack for DawgGuideAdapter {
    type View<'d> = DawgGuide<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let units = view.into_inner();
        units
            .iter()
            .flat_map(|&GuideEntry { child, sibling }| [child, sibling].into_iter())
            .collect()
    }
}
