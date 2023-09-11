use byteorder::ByteOrder;

mod adapter;
mod binpack;
pub mod mem_pack;
mod segments;
pub mod util;

pub use binpack::BinPack;

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

pub trait Suspend {
    type Suspended: Revive;

    fn suspend(&self) -> Self::Suspended;
}

pub trait Revive {
    type Revived<'d>;

    unsafe fn revive<'d>(&self, source_bin: &'d [u8]) -> Self::Revived<'d>;
}
