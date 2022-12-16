/*
 * AIVERO CONFIDENTIAL
 * __________________
 *
 *  [2017] - [2020] Aivero AS
 *  All Rights Reserved.
 *
 * NOTICE:  All information contained herein is, and remains
 * the property of Aivero AS and its suppliers,
 * if any. The intellectual and technical concepts contained
 * herein are proprietary to Aivero AS
 * and its suppliers and may be covered by EU,
 * patents in process, and are protected by trade secret or copyright law.
 * Dissemination of this information or reproduction of this material
 * is strictly forbidden unless prior written permission is obtained
 * from Aivero AS.
 */

use gst::glib;
use once_cell::sync::Lazy;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "framealigner",
        gst::DebugColorFlags::empty(),
        Some("Frame Aligner"),
    )
});

/// Struct that holds a Error struct for `framealigner`
#[derive(Debug, Clone)]
pub struct GstFrameAlignerError(pub String, Option<gst::DebugCategory>);
impl GstFrameAlignerError {
    pub(crate) fn new(msg: impl Into<String>) -> Self {
        Self(msg.into(), None)
    }
}
impl Error for GstFrameAlignerError {}
impl Display for GstFrameAlignerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "GstFrameAlignerError: {:?}", self.0)
    }
}
impl From<GstFrameAlignerError> for gst::FlowError {
    fn from(error: GstFrameAlignerError) -> Self {
        gst_error!(CAT, "{:?}", error);
        gst::FlowError::Error
    }
}
impl From<glib::BoolError> for GstFrameAlignerError {
    fn from(e: glib::BoolError) -> Self {
        Self::new(format!("A glib::BoolError occurred: {:?}", e))
    }
}
impl From<gst_depth_meta::RgbdError> for GstFrameAlignerError {
    fn from(e: gst_depth_meta::RgbdError) -> Self {
        Self::new(format!("A RgbdError occurred: {:?}", e))
    }
}
impl From<std::io::Error> for GstFrameAlignerError {
    fn from(e: std::io::Error) -> Self {
        Self::new(format!("Could not read config file: {:?}", e))
    }
}
impl From<serde_yaml::Error> for GstFrameAlignerError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::new(format!("Configuration file is invalid: {:?}", e))
    }
}
impl From<GstFrameAlignerError> for gst::LoggableError {
    fn from(e: GstFrameAlignerError) -> Self {
        gst::loggable_error!(e.1.unwrap_or(*CAT), "Frame Aligner error: {:?}", e)
    }
}

/// Struct containing caps information, which is useful for extracting frames from buffers
#[derive(Debug)]
pub struct CapsVideoInfo {
    /// Information about the depth video frame, e.g. height and width of the frame.
    pub depth_video_info: gst_video::VideoInfo,
    /// Information about the color video frame, e.g. height and width of the frame.
    pub color_video_info: gst_video::VideoInfo,
    /// Size the buffer allocated to read a block from the sink pads.
    pub sink_blocksize: usize,
    /// Size the buffer allocated to write a block to the src pads.
    pub src_blocksize: usize,
}

impl CapsVideoInfo {
    /// Function that creates a new CapsVideoInfo.
    /// # Arguments
    /// * `depth_video_info` - The depth video info.
    /// * `color_video_info` - The color video info, if present.
    /// * `sink_blocksize` - The size of the buffers on the sink pad.
    /// * `src_blocksize` - The size of the buffers on the src pad.
    /// # Returns
    /// A new instance of CapsVideoInfo.
    pub fn new(
        depth_video_info: gst_video::VideoInfo,
        color_video_info: gst_video::VideoInfo,
        sink_blocksize: usize,
        src_blocksize: usize,
    ) -> Self {
        Self {
            depth_video_info,
            color_video_info,
            sink_blocksize: sink_blocksize as usize,
            src_blocksize: src_blocksize as usize,
        }
    }
}
