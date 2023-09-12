mod completion;
mod dict;
mod guide;
mod list;

pub use self::completion::*;
pub use self::dict::*;
pub use self::guide::*;
pub use self::list::*;

#[cfg(test)]
mod tests {
    use byteorder::LittleEndian;

    use super::*;
    use crate::binpack::MemView;
    use crate::binpack::MemViewPack;
    use crate::binutils::hex_dump;
    use crate::binutils::Align64;
    use crate::collections::dawg::CompletionDawg;
    use crate::collections::dawg::GuideEntry;
    use crate::collections::dawg_value::WordValue;
    use crate::cryostasis::Revive;
    use crate::cryostasis::Suspend;

    #[rustfmt::skip]
    static LE_DAWG: Align64<[u8; 64 * 6]> = Align64([
        // segments meta
        0, 0,           // version
        0b_0000_0000,   // flags
        0,              // reserved1
        3, 0, 0, 0,     // segments_number
        192, 0, 0, 0, 0, 0, 0, 0,   // data_size
        // reserved2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        // segments start
        0, 0, 0, 0,     // start[0]
        64, 0, 0, 0,    // start[1]
        128, 0, 0, 0,   // start[2]
        //   ...
        0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        // segments size
        64, 0, 0, 0,    // size[0]
        12, 0, 0, 0,    // size[1]
        6, 0, 0, 0,     // size[2]
        //   ...
        0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        // segments data 1 / dawg meta
        0, 0, 0, 0,     // meta.root
        //   ...
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        // segments data 2 / dawg dictionary
        3, 2, 1, 0,     // units[0]
        7, 6, 5, 4,     // units[1]
        11, 10, 9, 8,   // units[2]
        //   ...
        0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,

        // segments data 3 / dawg guide
        0, 1,   // units[0]
        2, 3,   // units[1]
        4, 5,   // units[2]
        //   ...
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    fn guide_to_tuple(GuideEntry { child, sibling }: GuideEntry) -> (u8, u8) {
        (child, sibling)
    }

    fn check_dawg_bin(bin: &[u8]) {
        let dawg = CompletionDawgAdapter::<WordValue>::view(bin);
        check_dawg(dawg);
    }

    fn check_dawg(dawg: CompletionDawg<'_, WordValue>) {
        let (dict, guide) = dawg.into_inner();
        print!("\n\n");
        println!("{:?}", dict);
        println!("{:?}", guide);

        let dict = dict.into_inner();

        assert_eq!(dict.len(), 3);
        assert_eq!(dict[0].as_u32(), 0x0001_0203);
        assert_eq!(dict[1].as_u32(), 0x0405_0607);
        assert_eq!(dict[2].as_u32(), 0x0809_0A0B);

        let guide = guide.into_inner();

        assert_eq!(guide.len(), 3);
        assert_eq!(guide_to_tuple(guide[0]), (0x00, 0x01));
        assert_eq!(guide_to_tuple(guide[1]), (0x02, 0x03));
        assert_eq!(guide_to_tuple(guide[2]), (0x04, 0x05));

        print!("\n\n");
    }

    #[test]
    fn test_dawg_view_static() {
        check_dawg_bin(&*LE_DAWG);
    }

    #[test]
    fn test_dawg_packed() {
        let static_bin = &*LE_DAWG;
        let dawg = CompletionDawgAdapter::<WordValue>::view(static_bin);
        let packed = CompletionDawgAdapter::<WordValue>::pack::<LittleEndian>(&dawg);
        assert!(
            packed.as_ref() == static_bin,
            "packed ≠ source; \n{}\n{}",
            hex_dump("Source:", static_bin),
            hex_dump("packed", packed.as_ref())
        );
        check_dawg_bin(&packed);
    }

    #[test]
    fn test_dawg_owned() {
        let static_dawg = CompletionDawgAdapter::<WordValue>::view(&*LE_DAWG);
        let packed = CompletionDawgAdapter::<WordValue>::pack::<LittleEndian>(&static_dawg);
        let dawg = CompletionDawgAdapter::<WordValue>::view(&packed);
        let suspended = dawg.suspend();
        let revived = unsafe { suspended.revive(&packed) };
        check_dawg(revived);
    }
}
