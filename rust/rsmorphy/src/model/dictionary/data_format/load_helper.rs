use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use flate2::read::GzDecoder;

use crate::model::dictionary::LoadError;

#[derive(Debug, Clone)]
pub struct LoadHelper {
    dict_path: PathBuf,
}

#[derive(Debug)]
pub enum Reader {
    Plain(BufReader<File>),
    Gz(GzDecoder<File>),
}

impl Read for Reader {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Reader::Plain(r) => r.read(buf),
            Reader::Gz(r) => r.read(buf),
        }
    }
}

impl LoadHelper {
    pub fn new(p: impl AsRef<Path>) -> io::Result<Self> {
        let dict_path = p.as_ref().canonicalize()?;
        Ok(LoadHelper { dict_path })
    }

    #[inline(always)]
    pub fn path(&self, name: impl AsRef<Path>) -> PathBuf {
        self.dict_path.join(name)
    }

    #[inline(always)]
    fn read_plain(&self, path: &Path) -> io::Result<BufReader<File>> {
        log::debug!("Open {:?}", path);
        Ok(BufReader::new(File::open(&path)?))
    }

    #[inline(always)]
    fn read_gz(&self, path: &Path) -> io::Result<GzDecoder<File>> {
        log::debug!("Open {:?}", path);
        Ok(GzDecoder::new(File::open(path)?))
    }

    #[inline(always)]
    pub fn reader(&self, name: impl AsRef<Path>) -> io::Result<Reader> {
        self.reader_(self.path(name))
    }

    pub fn reader_(&self, mut path: PathBuf) -> io::Result<Reader> {
        let plain = match self.read_plain(&path) {
            Ok(plain) => return Ok(Reader::Plain(plain)),
            Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e)?,
            Err(e) => e,
        };

        let original_extention = path.extension().map_or(Ok(""), |ext| {
            ext.to_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "InvalidFilename"))
        })?;

        path.set_extension(format!("{}.gz", original_extention));

        match self.read_gz(&path) {
            Ok(gz) => return Ok(Reader::Gz(gz)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Err(plain)?,
            Err(e) => Err(e)?,
        }
    }

    pub fn json<T>(&self, name: impl AsRef<Path>) -> Result<T, LoadError>
    where
        for<'de> T: ::serde::Deserialize<'de>,
    {
        Ok(serde_json::from_reader(self.reader(name)?)?)
    }
}
