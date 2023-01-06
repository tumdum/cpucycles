use copy_dir::copy_dir;
use std::path::PathBuf;
use std::process::Command;

const PATH: &str = "./vendor/libcpucycles-20230105";

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    // We move to OUT_DIR because we can't tell configure
    // to build in OUT_DIR and we don't want to modify vendor
    // directory.
    let path = format!("{out_dir}/libcpucycles");
    let _ = std::fs::remove_dir_all(&path);
    copy_dir(PATH, &path).unwrap();
    Command::new("./configure")
        .arg(format!("--prefix={out_dir}"))
        .current_dir(&path)
        .status()
        .unwrap();
    Command::new("make")
        .arg("install")
        .current_dir(path)
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
    println!("cargo:rerun-if-changed={PATH}");
}
