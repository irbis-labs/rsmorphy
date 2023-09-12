use byteorder::ByteOrder;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binutils::bytes_to_ref;
use crate::collections::gram_sets::GramSets;

pub struct GramSetsAdapter;

impl MemView for GramSetsAdapter {
    type View<'d> = GramSets<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        let sets = unsafe { bytes_to_ref(bin) };

        GramSets::new(sets)
    }
}

impl MemViewPack for GramSetsAdapter {
    type View<'d> = GramSets<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let sets = (*view).into_inner();

        sets.iter()
            .copied()
            .flat_map(|set| {
                let mut buf = [0u8; 16];
                Order::write_u128(&mut buf, set.into_inner());
                buf.into_iter()
            })
            .collect()
    }
}
