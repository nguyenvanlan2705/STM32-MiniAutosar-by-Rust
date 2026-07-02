use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // Linker script tự viết (thay cho link.x của cortex-m-rt).
    File::create(out.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("link.x"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=link.x");

    println!("cargo:rustc-link-arg=-Tlink.x");
} 