use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let mut f = File::create("src/release.rs")
        .expect("Can't create a file");

    {
        let version = Command::new("rustc")
            .arg("--version")
            .output()
            .map(|o| String::from_utf8(o.stdout).expect("Can't convert from utf-8"))
            .expect("Can't get rustc version")
        ;
        writeln!(f, r"pub static RUSTC_VERSION: &str = {:?};", version)
            .expect("Can't write to a file");
    }
}
