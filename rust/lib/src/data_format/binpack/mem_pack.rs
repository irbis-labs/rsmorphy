use byteorder::ByteOrder;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::collections::dictionary::DictionaryView;
use crate::data_format::binpack::adapter::dictionary::DictionaryAdapter;

pub fn pack<Order: ByteOrder>(view: DictionaryView) -> BinPack {
    DictionaryAdapter::pack::<Order>(&view)
}

pub fn view(bin: &[u8]) -> DictionaryView {
    DictionaryAdapter::view(bin)
}
