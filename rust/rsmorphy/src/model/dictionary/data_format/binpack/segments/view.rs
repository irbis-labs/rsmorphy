use std::mem;

use crate::model::dictionary::data_format::binpack::segments::meta::SegmentsMeta;
use crate::model::dictionary::data_format::binpack::util::convert_slice;

#[derive(Debug, Clone, Copy)]
pub struct SegmentsView<'d> {
    meta: &'d SegmentsMeta,
    /// начало сегмента в блоке данных, выровненное на 64 байта.
    start: &'d [u32],
    /// точный размер данных в сегменте (их может быть меньше, чем выделено с учётом выравнивания).
    size: &'d [u32],
    /// непосредственно блок данных.
    data: &'d [u8],
}

impl<'d> SegmentsView<'d> {
    pub fn new(bin: &'d [u8]) -> Self {
        #[cfg(feature = "log_trace_model")]
        log::trace!("bin: {:?}", bin.as_ptr());
        #[cfg(all(feature = "log_trace_model", feature = "log_trace_large"))]
        log::trace!("bin: {:?}", bin);

        assert!(bin.len() >= mem::size_of::<SegmentsMeta>());
        let meta = unsafe { &*(&bin[0] as *const _ as *const SegmentsMeta) };

        #[cfg(feature = "log_trace_model")]
        log::trace!("meta: {:?}", meta as *const _);

        #[cfg(feature = "log_trace_model")]
        log::trace!("meta: {:?}", meta);

        assert_eq!(meta.version(), &[0, 0]);
        assert_eq!(meta.is_big_endian(), cfg!(target_endian = "big"));

        let seg_num = meta.segments_number() as usize;

        let start_begin: usize = 64;
        let start_end: usize = start_begin + seg_num * mem::size_of::<u32>();

        let size_begin = align(start_end, 64);
        let size_end = size_begin + seg_num * mem::size_of::<u32>();

        let data_begin = align(size_end, 64);
        let data_end = data_begin + meta.data_size() as usize;

        assert!(bin.len() >= data_end);

        let start = unsafe { convert_slice(&bin[start_begin..start_end]) }.unwrap();
        let size = unsafe { convert_slice(&bin[size_begin..size_end]) }.unwrap();
        let data = &bin[data_begin..];

        SegmentsView {
            meta,
            start,
            size,
            data,
        }
    }

    pub fn segments_number(&self) -> usize {
        self.meta.segments_number() as usize
    }

    #[cfg(test)]
    pub(super) fn start(&self) -> &[u32] {
        self.start
    }

    #[cfg(test)]
    pub(super) fn size(&self) -> &[u32] {
        self.size
    }

    #[cfg(test)]
    pub(super) fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn get(&self, index: usize) -> Option<&'d [u8]> {
        let start = self.start[index] as usize;
        let end = start + self.size[index] as usize;
        self.data.get(start..end)
    }
}

fn align(len: usize, align: usize) -> usize {
    let rem = len % align;
    len + (align - rem) % align
}
