use byteorder::ByteOrder;

use crate::data_struct::dawg::DawgDictionary;
use crate::model::dictionary::data_format::binpack::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::util::convert_slice;
use crate::model::dictionary::data_format::binpack::MemView;
use crate::model::dictionary::data_format::binpack::MemViewPack;

pub struct DawgDictionaryAdapter;

impl MemView for DawgDictionaryAdapter {
    type View<'d> = DawgDictionary<'d>;

    fn view(data: &[u8]) -> Self::View<'_> {
        let units = unsafe { convert_slice(data) }.unwrap();
        DawgDictionary::new(units)
    }
}

impl MemViewPack for DawgDictionaryAdapter {
    type View<'d> = DawgDictionary<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let units = view.into_inner();
        units
            .iter()
            .flat_map(|v| {
                let mut buf = [0u8; 4];
                Order::write_u32(&mut buf, v.as_u32());
                buf.into_iter()
            })
            .collect()
    }
}
