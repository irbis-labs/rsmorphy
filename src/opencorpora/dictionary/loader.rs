use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;

pub struct PathLoader {
    dict_path: PathBuf,
}

impl PathLoader {
    pub fn new<P>(p: P) -> Self
    where
        P: AsRef<Path>,
    {
        let dict_path = p.as_ref().into();
        PathLoader { dict_path }
    }

    pub fn path<S>(&self, name: S) -> PathBuf
    where
        S: AsRef<Path>,
    {
        self.dict_path.join(name)
    }

    pub fn reader<S>(&self, name: S) -> impl Read
    where
        S: AsRef<Path>,
    {
        let path = self.path(name);
        log::debug!("Open dict file {:?}", path);
        GzDecoder::new(File::open(&path).unwrap())
    }

    pub fn json<S, T>(&self, name: S) -> serde_json::Result<T>
    where
        S: AsRef<Path>,
            for<'de> T: ::serde::Deserialize<'de>,
    {
        serde_json::from_reader(self.reader(name))
    }
}
