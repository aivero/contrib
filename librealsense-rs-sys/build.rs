extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut config = pkg_config::Config::new();
    config.atleast_version("2.33.1");

    let path_args: Vec<String> = match config.probe("realsense2") {
        Ok(library) => library
            .include_paths
            .iter()
            .map(|x| ["-I", x.to_str().unwrap()].concat())
            .collect(),
        Err(err) => std::panic!("{}", err),
    };
    //pkg_config::Config::new()
    //    .atleast_version("1.0.22")
    //    .probe("libusb-1.0")
    //    .unwrap();
    //println!("cargo:rustc-flags=-l");
    //println!("cargo:rustc-link-lib=realsense-file");
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .rustified_enum("rs2_log_severity")
        .rustified_enum("rs2_camera_info")
        .rustified_enum("rs2_format")
        .rustified_enum("rs2_stream")
        .rustified_enum("rs2_distortion")
        .rustified_enum("rs2_option")
        .rustified_enum("rs2_sr300_visual_preset")
        .rustified_enum("rs2_rs400_visual_preset")
        .rustified_enum("rs2_l500_visual_preset")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .clang_args(path_args)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
