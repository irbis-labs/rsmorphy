mod builder;
mod meta;
mod view;

pub use self::builder::*;
pub use self::meta::*;
pub use self::view::*;

#[cfg(test)]
mod tests {
    use byteorder::LittleEndian;

    use super::*;
    use crate::binutils::Align64;

    #[rustfmt::skip]
    pub(super) static SAMPLE: Align64<[u8; 384]> = Align64([
        // version
        0, 0,
        // flags
        0b_0000_0000,
        // reserved1
        0,
        // segments_number
        3, 0, 0, 0,
        // data_size
        192, 0, 0, 0, 0, 0, 0, 0,
        // reserved2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // start
        0, 0, 0, 0, 64, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // size
        24, 0, 0, 0, 24, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // data 1
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // data 2
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // data 3
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    #[test]
    fn test_build() {
        let mut builder = SegmentsBuilder::new();
        builder.new_segment_from_slice(&[1; 24]);
        builder.new_segment_from_slice(&[2; 24]);
        builder.new_segment_from_slice(&[3; 24]);

        let bin = builder.build::<LittleEndian>();
        assert_eq!(&bin[..], &SAMPLE[..]);
    }

    #[test]
    fn test_view() {
        let view = SegmentsView::new(&*SAMPLE);
        assert_eq!(view.segments_number(), 3, "segments_number");
        assert_eq!(view.data().len(), 192, "data_size");
        assert_eq!(view.start()[0], 0, "start[0]");
        assert_eq!(view.start()[1], 64, "start[1]");
        assert_eq!(view.start()[2], 128, "start[2]");
        assert_eq!(view.size()[0], 24, "size[0]");
        assert_eq!(view.size()[1], 24, "size[1]");
        assert_eq!(view.size()[2], 24, "size[2]");
        assert_eq!(view.get(0).unwrap(), &[1; 24][..], "data[0]");
        assert_eq!(view.get(1).unwrap(), &[2; 24][..], "data[1]");
        assert_eq!(view.get(2).unwrap(), &[3; 24][..], "data[2]");
    }
}
