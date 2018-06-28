use std::fs::File;
use std::io::Read;
use std::path::Path;

use flate2::read::GzDecoder;

use util::u32_from_slice;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GuideEntry {
    pub child: u8,
    pub sibling: u8,
}

#[derive(Debug, Clone)]
pub struct Guide {
    /// Root index
    root: u32,
    pub units: Vec<GuideEntry>,
}

impl Guide {
    /// Reads a guide from a file.
    pub fn from_file(p: &Path) -> Self {
        Self::from_stream(&mut GzDecoder::new(File::open(p).unwrap()))
    }

    /// Reads a guide from an input stream.
    pub fn from_stream<T>(fp: &mut T) -> Self
    where
        T: Read,
    {
        let mut buf = [0u8; 4];
        fp.read_exact(&mut buf).unwrap();

        let base_size = u32::from_le(u32_from_slice(&buf[..])) as usize;
        let buf_size = base_size * 2;

        let mut buf: Vec<u8> = vec![0; buf_size];
        fp.read_exact(&mut buf[0..buf_size]).unwrap();
        assert_eq!(buf_size, buf.len());
        assert_eq!(buf_size, buf.capacity());

        let mut units: Vec<GuideEntry> = Vec::with_capacity(base_size);
        units.extend(buf.chunks(2).map(|ch| GuideEntry {
            child: ch[0],
            sibling: ch[1],
        }));
        assert_eq!(base_size, units.len());
        assert_eq!(base_size, units.capacity());

        let root = 0;

        Guide { root, units }
    }
}
