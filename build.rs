use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

fn main() {
    let release_path = &Path::new(&env::var("OUT_DIR").unwrap()).join("release.rs");
    let mut f = File::create(release_path).expect("Can't create a file");

    let res = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Can't get rustc version");

    let version = from_utf8(&res.stdout).expect("Can't convert from utf-8");

    writeln!(f, r"pub static RUSTC_VERSION: &str = {:?};", version).expect("Can't write to a file");
}
