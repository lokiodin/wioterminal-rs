use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        // Put `memory.x` in our output directory and ensure it's
        // on the linker search path.
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("memory.x"))
            .unwrap()
            .write_all(include_bytes!("memory.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=memory.x");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
