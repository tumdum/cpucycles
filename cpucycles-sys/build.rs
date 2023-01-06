use std::path::PathBuf;
use std::process::Command;

const PATH: &str = "./vendor/libcpucycles-20230105";

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    Command::new("./configure")
        .arg(format!("--prefix={out_dir}"))
        .current_dir(PATH)
        .status()
        .unwrap();
    Command::new("make")
        .arg("install")
        .current_dir(PATH)
        .status()
        .unwrap();

    let bindings = bindgen::Builder::default()
        .header(format!("{out_dir}/include/cpucycles.h"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static=cpucycles");

    println!("cargo:rerun-if-changed={PATH}/autogen");
    // println!("cargo:rerun-if-changed={PATH}/build");
    println!("cargo:rerun-if-changed={PATH}/command");
    println!("cargo:rerun-if-changed={PATH}/compilers");
    println!("cargo:rerun-if-changed={PATH}/configure");
    println!("cargo:rerun-if-changed={PATH}/cpucycles");
    println!("cargo:rerun-if-changed={PATH}/doc");
    println!("cargo:rerun-if-changed={PATH}/Makefile");
    println!("cargo:rerun-if-changed={PATH}/scripts-build");
    println!("cargo:rerun-if-changed={PATH}/version");
}
