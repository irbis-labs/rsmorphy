use byteorder::ByteOrder;
use byteorder::NativeEndian;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binutils::convert_slice;
use crate::collections::letters::Letters;

pub struct LettersAdapter;

impl MemView for LettersAdapter {
    type View<'d> = Letters<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        assert!(bin.len() % 4 == 0);

        // Check all chars are valid.
        for chunk in bin.chunks(4) {
            let chr = NativeEndian::read_u32(chunk);

            assert!(char::from_u32(chr).is_some(), "Invalid char: {}", chr);
        }

        let letters = unsafe { convert_slice(bin) }.unwrap();

        Letters::new(letters)
    }
}

impl MemViewPack for LettersAdapter {
    type View<'d> = Letters<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let letters = (*view).into_inner();

        letters
            .iter()
            .copied()
            .flat_map(|chr| {
                let mut buf = [0u8; 4];
                Order::write_u32(&mut buf, chr as u32);
                buf.into_iter()
            })
            .collect()
    }
}
