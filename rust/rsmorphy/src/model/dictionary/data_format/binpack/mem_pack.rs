use byteorder::ByteOrder;

use crate::model::dictionary::data_format::binpack::adapter::dictionary::DictionaryAdapter;
use crate::model::dictionary::data_format::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::MemView;
use crate::model::dictionary::data_format::binpack::MemViewPack;
use crate::model::dictionary::dictionary_view::DictionaryView;

pub fn pack<Order: ByteOrder>(view: DictionaryView) -> BinPack {
    DictionaryAdapter::pack::<Order>(&view)
}

pub fn view(bin: &[u8]) -> DictionaryView {
    DictionaryAdapter::view(bin)
}
