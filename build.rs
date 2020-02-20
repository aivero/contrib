extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut config = pkg_config::Config::new();
    config.atleast_version("2.17.0");

    let path_args: Vec<String> = match config.probe("realsense2") {
        Ok(library) => library
            .include_paths
            .iter()
            .map(|x| ["-I", x.to_str().unwrap()].concat())
            .collect(),
        Err(err) => std::panic!(err),
    };
    pkg_config::Config::new()
        .atleast_version("1.0.22")
        .probe("libusb-1.0")
        .unwrap();
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-link-lib=realsense-file");
    println!("cargo:rustc-link-lib=tm");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .rustified_enum("rs2_log_severity")
        .rustified_enum("rs2_camera_info")
        .rustified_enum("rs2_format")
        .rustified_enum("rs2_stream")
        .rustified_enum("rs2_distortion")
        .clang_args(path_args)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
