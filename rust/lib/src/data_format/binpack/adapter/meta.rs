use byteorder::ByteOrder;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binutils::bytes_to_ref;
use crate::binutils::ref_to_bytes;
use crate::collections::dictionary_meta::DictionaryMeta;

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
