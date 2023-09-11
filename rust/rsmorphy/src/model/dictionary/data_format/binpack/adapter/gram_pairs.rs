use byteorder::ByteOrder;

use crate::data_struct::gram_pairs::GramPairs;
use crate::model::dictionary::data_format::binpack::util::convert_slice;
use crate::model::dictionary::data_format::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::MemView;
use crate::model::dictionary::data_format::binpack::MemViewPack;

pub struct GramPairsAdapter;

impl MemView for GramPairsAdapter {
    type View<'d> = GramPairs<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let replaces = unsafe { convert_slice(bin) }.unwrap();

        GramPairs::new(replaces)
    }
}

impl MemViewPack for GramPairsAdapter {
    type View<'d> = GramPairs<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let replaces = (*view).into_inner();

        let mut builder = BinPack::builder();
        builder.extend_from_bytes(unsafe { convert_slice(replaces) }.unwrap());
        builder.build()
    }
}
