use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let mut f = File::create("src/release.rs").unwrap();

    {
        let version = Command::new("rustc")
            .arg("--version")
            .output()
            .map(|o| String::from_utf8(o.stdout).unwrap())
            .expect("Can't get rustc version")
        ;
        writeln!(f, r"pub const RUSTC_VERSION: &'static str = {:?};", version).unwrap();
    }
}
