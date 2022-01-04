extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("0xhaust/asm.c")
        .file("0xhaust/sim.c")
        .warnings(false)
        .compile("0xhaust");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .no_copy("SimState_t")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
