#![allow(linker_messages)]

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        println!("cargo:rerun-if-changed=assets/icon.ico");
        println!("cargo:rerun-if-changed=app.rc");
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        println!("cargo:rustc-link-arg={}\\assets\\app.o", manifest_dir);
    }
}
