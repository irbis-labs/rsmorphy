use byteorder::ByteOrder;

use crate::model::dictionary::data_format::binpack::util::bytes_to_ref;
use crate::model::dictionary::data_format::binpack::util::ref_to_bytes;
use crate::model::dictionary::data_format::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::MemView;
use crate::model::dictionary::data_format::binpack::MemViewPack;
use crate::model::dictionary::meta::DictionaryMeta;

pub struct DictionaryMetaAdapter;

impl MemView for DictionaryMetaAdapter {
    type View<'d> = DictionaryMeta<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let meta = unsafe { bytes_to_ref(bin) };

        DictionaryMeta::new(meta)
    }
}

impl MemViewPack for DictionaryMetaAdapter {
    type View<'d> = DictionaryMeta<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        // Copy of the DictionaryMetaOwned
        let mut meta = *(*view).into_inner();

        // NOTE Fix endianess for future fields with size greater than u8.

        let bytes = unsafe { ref_to_bytes(&mut meta) };
        bytes.into()
    }
}
