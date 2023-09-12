use byteorder::ByteOrder;
use byteorder::NativeEndian;

use crate::binpack::BinPack;
use crate::binpack::MemView;
use crate::binpack::MemViewPack;
use crate::binutils::convert_slice;
use crate::collections::letter_pairs::LetterPairs;

pub struct LetterPairsAdapter;

impl MemView for LetterPairsAdapter {
    type View<'d> = LetterPairs<'d>;

    fn view(bin: &[u8]) -> Self::View<'_> {
        assert!(bin.len() % 8 == 0);

        // Check all chars are valid.
        for chunk in bin.chunks(4) {
            let chr = NativeEndian::read_u32(chunk);

            assert!(char::from_u32(chr).is_some(), "Invalid char: {}", chr);
        }

        let len = bin.len() / 2;
        let (chars, subst) = bin.split_at(len);

        let chars = unsafe { convert_slice(chars) }.unwrap();
        let subst = unsafe { convert_slice(subst) }.unwrap();

        LetterPairs::new(chars, subst)
    }
}

impl MemViewPack for LetterPairsAdapter {
    type View<'d> = LetterPairs<'d>;

    fn pack<Order: ByteOrder>(view: &Self::View<'_>) -> BinPack {
        let (chars, subst) = (*view).into_inner();

        chars
            .iter()
            .copied()
            .chain(subst.iter().copied())
            .flat_map(|chr| {
                let mut buf = [0u8; 4];
                Order::write_u32(&mut buf, chr as u32);
                buf.into_iter()
            })
            .collect()
    }
}
