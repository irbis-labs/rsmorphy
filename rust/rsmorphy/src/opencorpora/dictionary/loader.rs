use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use flate2::read::GzDecoder;

use crate::opencorpora::dictionary::LoadError;

pub struct PathLoader {
    dict_path: PathBuf,
}

impl PathLoader {
    pub fn new(p: impl Into<PathBuf>) -> Self {
        let dict_path = p.into();
        PathLoader { dict_path }
    }

    pub fn path(&self, name: impl AsRef<Path>) -> PathBuf {
        self.dict_path.join(name)
    }

    pub fn reader(&self, name: impl AsRef<Path>) -> Result<impl Read, LoadError> {
        let path = self.path(name);
        log::debug!("Open dict file {:?}", path);
        Ok(GzDecoder::new(File::open(&path)?))
    }

    pub fn json<T>(&self, name: impl AsRef<Path>) -> Result<T, LoadError>
    where
        for<'de> T: ::serde::Deserialize<'de>,
    {
        Ok(serde_json::from_reader(self.reader(name)?)?)
    }
}
