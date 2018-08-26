use std::fs::File;
use std::io::Read;
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::GzDecoder;

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
        Self::from_reader(&mut GzDecoder::new(File::open(p).unwrap()))
    }

    /// Reads a guide from an input stream.
    pub fn from_reader<T>(fp: &mut T) -> Self
    where
        T: Read,
    {
        let base_size = fp.read_u32::<LittleEndian>().unwrap();
        let units: Vec<GuideEntry> = (0..base_size)
            .map(|_| (fp.read_u8().unwrap(), fp.read_u8().unwrap()))
            .map(|(child, sibling)| GuideEntry { child, sibling })
            .collect();

        let root = 0;

        Guide { root, units }
    }
}
