use super::error::*;
use super::utilities::*;
use libk4a::utilities::*;
use libk4a::{DeviceConfiguration, RecordConfiguration};
pub(crate) use std::convert::TryFrom;

/// A struct that helps during fixation of CAPS.
pub(crate) struct StreamProperties {
    /// Color format in readable format.
    pub(crate) color_format: &'static str,
    /// Depth resolution in readable format.
    pub(crate) depth_resolution: Resolution,
    /// IR resolution in readable format.
    pub(crate) ir_resolution: Resolution,
    /// Color resolution in readable format.
    pub(crate) color_resolution: Resolution,
    /// Framerate of the camera streams in readable format.
    pub(crate) framerate: i32,
}

impl TryFrom<&DeviceConfiguration> for StreamProperties {
    type Error = K4aSrcError;
    fn try_from(record_configuration: &DeviceConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            color_format: k4a_image_format_to_gst_video_format(record_configuration.color_format)?,
            // Note: `.unwrap_or_default()` for resolutions are utilised to avoid error if any of the streams is disabled.
            depth_resolution: depth_mode_to_ir_resolution(record_configuration.depth_mode)
                .unwrap_or_default(),
            ir_resolution: depth_mode_to_depth_resolution(record_configuration.depth_mode)
                .unwrap_or_default(),
            color_resolution: color_resolution_to_resolution(record_configuration.color_resolution)
                .unwrap_or_default(),
            framerate: fps_to_i32(record_configuration.camera_fps),
        })
    }
}

impl TryFrom<&RecordConfiguration> for StreamProperties {
    type Error = K4aSrcError;
    fn try_from(record_configuration: &RecordConfiguration) -> Result<Self, Self::Error> {
        Ok(Self {
            color_format: k4a_image_format_to_gst_video_format(record_configuration.color_format)?,
            // Note: `.unwrap_or_default()` for resolutions are utilised to avoid error if any of the streams is disabled.
            depth_resolution: depth_mode_to_ir_resolution(record_configuration.depth_mode)
                .unwrap_or_default(),
            ir_resolution: depth_mode_to_depth_resolution(record_configuration.depth_mode)
                .unwrap_or_default(),
            color_resolution: color_resolution_to_resolution(record_configuration.color_resolution)
                .unwrap_or_default(),
            framerate: fps_to_i32(record_configuration.camera_fps),
        })
    }
}
