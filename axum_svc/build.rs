use std::env;
fn main() {

    println!("cargo:rerun-if-changed=src/main.rs");

    // let out_dir = env::var("OUT_DIR").unwrap();
    env::set_var("OUT_DIR","./bin");
    env::set_var("CARGO_TARGET_DIR","./bin");
    println!("hello, build.rs");

}