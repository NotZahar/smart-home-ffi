use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=../smart-socket-ffi/src/lib.rs");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR must be set by Cargo"));
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("Cargo OUT_DIR should be inside target/{profile}/build");

    println!("cargo:rustc-link-search=native={}", profile_dir.display());
    println!(
        "cargo:rustc-link-search=native={}",
        profile_dir.join("deps").display()
    );
    println!("cargo:rustc-link-lib=static=smart_socket_ffi");
}
