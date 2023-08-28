use std::io;
use std::io::Read;

use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use serde::Deserialize;
use serde::Serialize;

use super::guide_entry::GuideEntry;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Guide {
    /// Root index
    root: u32,
    units: Vec<GuideEntry>,
}

impl Guide {
    /// Reads a guide from an input stream.
    pub fn from_reader(mut reader: impl Read) -> io::Result<Self> {
        let root = 0;

        let units_count = reader.read_u32::<LittleEndian>()?;
        if units_count == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Guide is empty"));
        }

        let units = (0..units_count)
            .map(|_| Ok(GuideEntry::new(reader.read_u8()?, reader.read_u8()?)))
            .collect::<io::Result<_>>()?;

        Ok(Guide { root, units })
    }

    #[inline(always)]
    pub(super) fn unit(&self, index: u32) -> &GuideEntry {
        let index = index as usize;
        debug_assert!(index < self.units.len(), "index out of bounds");
        unsafe { self.units.get_unchecked(index) }
    }
}
