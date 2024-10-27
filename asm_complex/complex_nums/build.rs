extern crate bindgen;
extern crate cc;

use bindgen::builder;
use std::path::PathBuf;

fn main() {
    cc::Build::new().file("complex_numbers/complex.c")
    .compiler("clang").target("thumbv7em-none-eabihf")
    .flag("-mthumb").flag("-march=armv7e-m").flag("-mfloat-abi=hard")
    .flag("-mfpu=fpv4-sp-d16").flag("-nostdlib").compile("complex_numbers-c");

    let bindings =  builder()
        .header("wrapper.h")
        .use_core()
        .ctypes_prefix("core::ffi")
        .derive_default(true)
        .generate()
        .expect("bindings couldn't be genereated! :(");

    
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
