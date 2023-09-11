use std::io;
use std::io::Write;

use crate::model::dictionary::data_format::binpack::binpack::BinChunk;
use crate::model::dictionary::data_format::binpack::binpack::BinPack;

#[derive(Debug, Clone, Default)]
pub struct BinPackBuilder {
    data: Vec<BinChunk>,
    len: usize,
}

impl BinPackBuilder {
    #[inline(always)]
    pub fn new() -> Self {
        BinPackBuilder::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = (capacity + BinChunk::SIZE - 1) / BinChunk::SIZE;
        let data = Vec::with_capacity(capacity);
        BinPackBuilder { data, len: 0 }
    }

    #[inline(always)]
    pub fn build(self) -> BinPack {
        let data = self.data.into_boxed_slice();
        let len = self.len;
        BinPack { data, len }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn reserve(&mut self, additional: usize) {
        let unused_chunks = self.data.capacity() - self.data.len();
        let unused_bytes = unused_chunks * BinChunk::SIZE + calc_pad(self.len, BinChunk::SIZE);
        let additional_bytes = additional.saturating_sub(unused_bytes);
        let additional_chunks = (additional_bytes + BinChunk::SIZE - 1) / BinChunk::SIZE;
        if additional_chunks > 0 {
            self.data.reserve(additional_chunks);
        }
    }

    #[inline(always)]
    pub fn pad(&mut self) {
        let len = self.data.len() * BinChunk::SIZE;
        self.len = len;
    }

    #[inline(always)]
    pub fn extend_from_bytes(&mut self, slice: &[u8]) {
        self.write(slice).unwrap();
    }
}

impl Extend<u8> for BinPackBuilder {
    #[inline(always)]
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        let iter = iter.into_iter();

        let (lower_bound, _) = iter.size_hint();
        if lower_bound > 0 {
            self.data.reserve(lower_bound);
        }

        iter.fold((), |_, byte| {
            // TODO optimize in the same way as in BinPackBuilder::write?
            self.write(&[byte]).unwrap();
        });
    }
}

impl Write for BinPackBuilder {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let remnant = calc_pad(self.len, BinChunk::SIZE);

        if remnant > 0 {
            let buf_rem = remnant.min(buf.len());

            let last_chunk = {
                debug_assert!(
                    self.data.len() > 0,
                    "assert self.data.len(): {} > 0",
                    self.data.len()
                );

                // SAFETY: if remnant > 0, then we have at least one chunk in self.data.
                let index = self.data.len() - 1;
                // SAFETY: index is always in bounds.
                unsafe { self.data.get_unchecked_mut(index) }
            };

            let begin = self.len % BinChunk::SIZE;
            let end = begin + buf_rem;

            debug_assert!(
                end <= BinChunk::SIZE,
                "assert end: {} <= {}",
                end,
                BinChunk::SIZE
            );

            last_chunk[begin..end].copy_from_slice(&buf[..buf_rem]);
            self.len += buf_rem;

            if buf_rem == buf.len() {
                return Ok(buf_rem);
            }
        }

        let buf = &buf[remnant..];
        let rem = buf.len() % BinChunk::SIZE;
        let aligned_len = buf.len() - rem;

        debug_assert_eq!(
            aligned_len % BinChunk::SIZE,
            0,
            "assert aligned_len: {} % {} == 0",
            aligned_len,
            BinChunk::SIZE
        );

        self.data.extend(
            buf[..aligned_len]
                .chunks(BinChunk::SIZE)
                .map(BinChunk::copy_from_slice_unchecked),
        );

        if rem > 0 {
            let mut chunk = BinChunk::zeroed();
            chunk[..rem].copy_from_slice(&buf[aligned_len..]);
            self.data.push(chunk);
        }

        self.len += buf.len();
        Ok(buf.len())
    }

    #[inline(always)]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[inline(always)]
fn calc_pad(size: usize, align: usize) -> usize {
    let rem = size % align;
    (align - rem) % align
}
