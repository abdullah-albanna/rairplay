use std::{path::Path, process::Command};

fn main() {
    let mut build = cc::Build::new();
    let out = std::env::var("OUT_DIR").unwrap();
    let shairplay = Path::new(&out).join("shairplay");

    if !shairplay.exists() {
        let command = Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("https://github.com/juhovh/shairplay")
            .arg(&shairplay)
            .status()
            .unwrap();
        assert!(command.success());
    }

    let playfair_dir = shairplay
        .join("src")
        .join("lib")
        .join("playfair")
        .join("*.c");

    let playfair_dir_str = playfair_dir.to_str().unwrap();

    for entry in glob::glob(playfair_dir_str).unwrap() {
        build.file(entry.unwrap());
    }
    build.cargo_warnings(false).compile("fairplay3");
    println!("cargo:rerun-if-changed={}", shairplay.to_str().unwrap());
}
