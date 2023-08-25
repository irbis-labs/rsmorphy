use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() -> io::Result<()> {
    let dict_path = Path::new("./data").canonicalize()?;

    let release_path = &Path::new(&env::var("OUT_DIR").unwrap()).join("release.rs");
    let mut f = File::create(release_path).expect("Can't create a file");

    writeln!(f, r"pub const DICT_PATH: &str = {:?};", dict_path)?;

    Ok(())
}
