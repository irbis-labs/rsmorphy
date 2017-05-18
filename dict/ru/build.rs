use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // let out_dir = Path::new(&out_dir);

    let dict_path = Path::new("./data").canonicalize().unwrap();
//    let dict_path = assets_dir.join().canonicalize().unwrap();

    let mut f = File::create("src/release.rs").unwrap();

    {
        let version = match Command::new("rustc")
            .arg("--version")
            .output() {
                Ok(o) => String::from(from_utf8(&o.stdout).unwrap().trim()),
                Err(_) => String::new(),
            };
        writeln!(f, r"pub const RUSTC_VERSION: &'static str = {:?};", version).unwrap();
        //writeln!(f, r"pub const OUT_DIR: &'static str = {:?};", out_dir).unwrap();
        writeln!(f, r"pub const DICT_PATH: &'static str = {:?};", dict_path).unwrap();
    }
}
