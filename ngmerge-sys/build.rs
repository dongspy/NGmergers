extern crate bindgen;
use cc;

use std::env;
use std::path::PathBuf;

const FILES:&[&str] = &[
    "NGmerge/NGmerge.c",
    "NGmerge/NGmerge.h"
];

fn main() {
    let lib_path = PathBuf::from(env::current_dir().unwrap().join("NGmerge/build"));
    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-search=/usr/local/Cellar/libomp/12.0.1/lib/");
    println!("cargo:rustc-link-lib=static=ngmerge");
    println!("cargo:rustc-link-lib=dylib=z"); //omp
    println!("cargo:rustc-link-lib=dylib=omp");

    for file in FILES {
        println!("cargo:rerun-if-changed={}", file);
    }

    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .file("/Users/lipidong/learn/rust/NGmergers/ngmerge-sys/NGmerge/NGmerge.c")
        .flag("-fopenmp")
        .flag("-std=gnu99")
        .flag("-Wall")
        .out_dir(lib_path)
        // .ar_flag("/Users/lipidong/learn/rust/NGmergers/ngmerge-sys/NGmerge/build/libaa.a")
        .compile("libngmerge.a");

    let bindings = bindgen::Builder::default()
        .header("NGmerge/NGmerge.h")
        // .whitelist_function("hello_from_c")
        .generate()
        .expect("unable to generate hello bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
