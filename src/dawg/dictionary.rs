use std::{fs::File, io::Read, path::Path};

use boolinator::Boolinator;
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};

use crate::dawg::units;

/// Dictionary class for retrieval and binary I/O.
#[derive(Deserialize, Serialize)]
#[derive(Debug, Clone)]
pub struct Dictionary {
    /// Root index
    pub root: u32,
    pub units: Vec<u32>,
}

impl Dictionary {
    /// Reads a dictionary from a file.
    pub fn from_file(p: &Path) -> Self {
        Self::from_reader(&mut GzDecoder::new(File::open(p).unwrap()))
    }

    /// Reads a dictionary from an input stream.
    pub fn from_reader<T>(reader: &mut T) -> Self
    where
        T: Read,
    {
        let size = reader.read_u32::<LittleEndian>().unwrap();
        let units = (0..size)
            .map(|_| reader.read_u32::<LittleEndian>().unwrap())
            .collect();

        let root = 0;

        Dictionary { root, units }
    }

    /// Checks if a given index is related to the end of a key.
    pub fn has_value(&self, index: u32) -> bool {
        units::has_leaf(self.units[index as usize])
    }

    /// Gets a value from a given index.
    pub fn value(&self, index: u32) -> u32 {
        let offset = units::offset(self.units[index as usize]);
        let value_index = (index ^ offset) & units::PRECISION_MASK;
        units::value(self.units[value_index as usize])
    }

    /// Gets a value from a given index if a given index is related to the end of a key.
    pub fn try_value(&self, index: u32) -> Option<u32> {
        self.has_value(index).as_some_from(|| self.value(index))
    }

    /// Exact matching.
    pub fn contains(&self, key: &str) -> bool {
        self.follow_bytes(key, self.root)
            .map(|index| self.has_value(index))
            .unwrap_or(false)
    }

    /// Exact matching (returns value)
    pub fn find(&self, key: &str) -> Option<u32> {
        self.follow_bytes(key, self.root)
            .and_then(|index| self.try_value(index))
    }

    /// Follows a transition
    pub fn follow_char(&self, label: u8, index: u32) -> Option<u32> {
        log::trace!(
            r#"Dictionary::follow_char() label: {:x}, index = {:x} "#,
            label,
            index
        );
        let unit = self.units[index as usize];
        log::trace!(r#"Dictionary::follow_char() unit: {:x} "#, unit);
        let offset = units::offset(unit);
        log::trace!(r#"Dictionary::follow_char() offset: {:x} "#, offset);
        let next_index = (index ^ offset ^ u32::from(label)) & units::PRECISION_MASK;
        let leaf_label = units::label(self.units[next_index as usize]);
        log::trace!(r#"Dictionary::follow_char() leaf_label: {:x} "#, leaf_label);
        if leaf_label == u32::from(label) {
            return Some(next_index);
        }
        None
    }

    /// Follows transitions.
    pub fn follow_bytes(&self, key: &str, mut index: u32) -> Option<u32> {
        for &ch in key.as_bytes() {
            index = self.follow_char(ch, index)?;
        }
        Some(index)
    }
}
