// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.
#![doc(
    html_logo_url = "https://2iexix426zex1jl94h409m26-wpengine.netdna-ssl.com/wp-content/uploads/2018/09/jira-logo-transparent.png"
)]

extern crate librealsense2_sys as rs2;

pub mod config;
pub mod context;
pub mod device;
pub mod error;
pub mod extrinsics;
pub mod frame;
pub mod high_level_utils;
pub mod intrinsics;
pub mod log;
mod low_level_utils;
pub mod metadata;
pub mod pipeline;
pub mod pipeline_profile;
pub mod processing;
pub mod sensor;
pub mod stream_profile;

// Expose types for external use
pub use config::rs2_format;
pub use config::rs2_stream;
pub use device::rs2_camera_info;
pub use log::rs2_log_severity;
