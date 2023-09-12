use byteorder::ByteOrder;

use super::BinPack;

pub trait MemView {
    type View<'d>;

    fn view(bin: &[u8]) -> Self::View<'_>;
}

pub trait MemViewPack {
    type View<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack;
}

// pub trait MemViewImport {
//     fn fix_endianness<Order: ByteOrder>(bin: &mut [u8]);
// }
