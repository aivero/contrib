extern crate gst;
extern crate gst_base;
extern crate gst_sys;

extern crate capnp;
#[allow(clippy::all)]
pub(crate) mod camera_meta_capnp {
    #![allow(dead_code)]
    #![allow(clippy::redundant_field_names)]
    include!(concat!(env!("OUT_DIR"), "/camera_meta_capnp.rs"));
}

mod common;

pub mod camera_meta;
pub mod dddq_roi_tags;
pub mod rgbd;

pub use camera_meta::*;
pub use rgbd::*;
