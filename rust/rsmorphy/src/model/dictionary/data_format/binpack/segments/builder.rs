use std::io::Write;

use byteorder::ByteOrder;
use byteorder::WriteBytesExt;

use crate::model::dictionary::data_format::binpack::binpack::BinPack;
use crate::model::dictionary::data_format::binpack::binpack::BinPackBuilder;
use crate::model::dictionary::data_format::binpack::segments::meta::SegmentsMeta;

const SEGMENT_ALIGN: usize = 64;

#[derive(Debug, Clone)]
pub struct SegmentsBuilder {
    /// начало сегмента в блоке данных, выровненное на 64 байта.
    pub(super) start: Vec<u32>,
    /// точный размер данных в сегменте (их может быть меньше, чем выделено с учётом выравнивания).
    pub(super) size: Vec<u32>,
    /// непосредственно блок данных.
    pub(super) data: Vec<u8>,
}

impl SegmentsBuilder {
    pub fn new() -> Self {
        SegmentsBuilder {
            start: Vec::new(),
            size: Vec::new(),
            data: Vec::new(),
        }
    }

    /// Создаёт новый сегмент из среза данных и выравнивает его на 64 байта.
    pub fn new_segment_from_slice(&mut self, data: &[u8]) -> usize {
        let seg_num = self.start.len();

        let start = self.data.len();
        let size = data.len();
        let pad = padding(data, SEGMENT_ALIGN);

        self.data.extend_from_slice(data);
        self.data.extend((0..pad).map(|_| 0));

        self.start.push(start as u32);
        self.size.push(size as u32);

        seg_num
    }

    pub fn build<Order: ByteOrder>(self) -> BinPack {
        let Self { start, size, data } = self;
        debug_assert_eq!(start.len(), size.len());

        let meta = SegmentsMeta::new::<Order>(start.len() as u32, data.len() as u64);

        let mut bin = BinPackBuilder::new();
        meta.write::<Order>(bin.by_ref()).unwrap();
        debug_assert_eq!(bin.len(), 64);

        for v in start {
            bin.write_u32::<Order>(v).unwrap();
        }
        bin.pad();

        for v in size {
            bin.write_u32::<Order>(v).unwrap();
        }
        bin.pad();

        bin.extend_from_bytes(&data);
        bin.pad();

        bin.build()
    }
}

fn padding(data: &[u8], align: usize) -> usize {
    calc_pad(data.len(), align)
}

#[inline(always)]
fn calc_pad(size: usize, align: usize) -> usize {
    let rem = size % align;
    (align - rem) % align
}
