use byteorder::ByteOrder;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binutils::convert_slice;
use crate::collections::dawg::DawgGuide;
use crate::collections::dawg::GuideEntry;

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
