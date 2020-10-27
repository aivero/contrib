// Aivero
// Copyright (C) <2019> Aivero
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Library General Public
// License as published by the Free Software Foundation; either
// version 2 of the License, or (at your option) any later version.
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Library General Public License for more details.
// You should have received a copy of the GNU Library General Public
// License along with this library; if not, write to the
// Free Software Foundation, Inc., 51 Franklin St, Fifth Floor,
// Boston, MA 02110-1301, USA.

use crate::error::K4aSrcError;
use glib::StaticType;
use k4a::{DepthMode, ImageFormat};
use std::convert::TryFrom;

/// Represents the Azure Kinect's color format and is used here to implement it as a GStreamer property.
/// It is a small wrapper around the k4a::ImageFormat enum, storing only the
/// K4A_IMAGE_FORMAT_COLOR_* part of it.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, GEnum)]
#[repr(u32)]
#[genum(type_name = "GstK4aColorFormat")]
pub enum K4aColorFormat {
    #[genum(name = "MJPG", nick = "mjpg")]
    MJPG = 0,
    #[genum(name = "NV12 (720p only)", nick = "nv12")]
    NV12 = 1,
    #[genum(name = "YUV2 (720p only)", nick = "yuv2")]
    YUV2 = 2,
    #[genum(name = "BGRA32", nick = "bgra32")]
    BGRA32 = 3,
}

/// Try to convert a `k4a::ImageFormat` into a [K4aColorFormat](enum.K4aColorFormat.html). This is a
/// TryFrom, as `k4a::ImageFormat` is wider than [K4aColorFormat](enum.K4aColorFormat.html), as it
/// also holds image formats for other streams.
impl TryFrom<k4a::ImageFormat> for K4aColorFormat {
    type Error = K4aSrcError;

    fn try_from(value: k4a::ImageFormat) -> Result<Self, Self::Error> {
        match value {
            ImageFormat::K4A_IMAGE_FORMAT_COLOR_NV12 => Ok(K4aColorFormat::NV12),
            ImageFormat::K4A_IMAGE_FORMAT_COLOR_YUY2 => Ok(K4aColorFormat::YUV2),
            ImageFormat::K4A_IMAGE_FORMAT_COLOR_MJPG => Ok(K4aColorFormat::MJPG),
            ImageFormat::K4A_IMAGE_FORMAT_COLOR_BGRA32 => Ok(K4aColorFormat::BGRA32),
            _ => Err(K4aSrcError::Failure("Unsupported ImageFormat conversion")),
        }
    }
}
/// Convert a [K4aColorFormat](enum.K4aColorFormat.html) into a `k4a::ImageFormat`. This can be
/// converted directly, as all values represented by [K4aColorFormat](enum.K4aColorFormat.html) is
/// also represented by `k4a::ImageFormat`.
impl From<K4aColorFormat> for k4a::ImageFormat {
    fn from(cf: K4aColorFormat) -> Self {
        match cf {
            K4aColorFormat::NV12 => ImageFormat::K4A_IMAGE_FORMAT_COLOR_NV12,
            K4aColorFormat::BGRA32 => ImageFormat::K4A_IMAGE_FORMAT_COLOR_BGRA32,
            K4aColorFormat::MJPG => ImageFormat::K4A_IMAGE_FORMAT_COLOR_MJPG,
            K4aColorFormat::YUV2 => ImageFormat::K4A_IMAGE_FORMAT_COLOR_YUY2,
        }
    }
}

/// Represents the Azure Kinect's color resolution and is used here to implement it as a GStreamer property.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, GEnum)]
#[repr(u32)]
#[genum(type_name = "GstK4aColorResolution")]
pub enum K4aColorResolution {
    #[genum(name = "720p: 720p resolution", nick = "720p")]
    C720p = 0,
    #[genum(name = "1080p: 1080p resolution", nick = "1080p")]
    C1080p = 1,
    #[genum(name = "1440p: 1440p resolution", nick = "1440p")]
    C1440p = 2,
    #[genum(name = "1536p: 1536p resolution", nick = "1536p")]
    C1536p = 3,
    #[genum(name = "2160p: 2160p resolution", nick = "2160p")]
    C2160p = 4,
    #[genum(name = "3072p: 3072p resolution", nick = "3072p")]
    C3072p = 5,
}

/// Try to convert a `k4a::ColorResolution` into a [K4aColorResolution](enum.K4aColorResolution.html). This is a
/// TryFrom, as `k4a::ColorResolution` is wider than [K4aColorResolution](enum.K4aColorResolution.html).
impl TryFrom<k4a::ColorResolution> for K4aColorResolution {
    type Error = K4aSrcError;

    fn try_from(res: k4a::ColorResolution) -> Result<Self, Self::Error> {
        match res {
            k4a::ColorResolution::K4A_COLOR_RESOLUTION_720P => Ok(K4aColorResolution::C720p),
            k4a::ColorResolution::K4A_COLOR_RESOLUTION_1080P => Ok(K4aColorResolution::C1080p),
            k4a::ColorResolution::K4A_COLOR_RESOLUTION_1440P => Ok(K4aColorResolution::C1440p),
            k4a::ColorResolution::K4A_COLOR_RESOLUTION_1536P => Ok(K4aColorResolution::C1536p),
            k4a::ColorResolution::K4A_COLOR_RESOLUTION_2160P => Ok(K4aColorResolution::C2160p),
            k4a::ColorResolution::K4A_COLOR_RESOLUTION_3072P => Ok(K4aColorResolution::C3072p),
            _ => Err(K4aSrcError::Failure(
                "Unsupported k4a color resolution conversion",
            )),
        }
    }
}
/// Convert a [K4aColorResolution](enum.K4aColorResolution.html) into a `k4a::ColorResolution`. This can be
/// converted directly, as all values represented by [K4aColorResolution](enum.K4aColorResolution.html) is
/// also represented by `k4a::ColorResolution`.
impl From<K4aColorResolution> for k4a::ColorResolution {
    fn from(cr: K4aColorResolution) -> Self {
        match cr {
            K4aColorResolution::C720p => k4a::ColorResolution::K4A_COLOR_RESOLUTION_720P,
            K4aColorResolution::C1080p => k4a::ColorResolution::K4A_COLOR_RESOLUTION_1080P,
            K4aColorResolution::C1440p => k4a::ColorResolution::K4A_COLOR_RESOLUTION_1440P,
            K4aColorResolution::C1536p => k4a::ColorResolution::K4A_COLOR_RESOLUTION_1536P,
            K4aColorResolution::C2160p => k4a::ColorResolution::K4A_COLOR_RESOLUTION_2160P,
            K4aColorResolution::C3072p => k4a::ColorResolution::K4A_COLOR_RESOLUTION_3072P,
        }
    }
}

/// Represents the Azure Kinect's depth mode and is used here to implement it as a GStreamer property.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, GEnum)]
#[repr(u32)]
#[genum(type_name = "GstK4aDepthMode")]
pub enum K4aDepthMode {
    #[genum(name = "NFOV 2x2 Binned", nick = "nfov_2x2_binned")]
    Nfov2x2Binned,
    #[genum(name = "NFOV Unbinned", nick = "nfov_unbinned")]
    NfovUnbinned,
    #[genum(name = "WFOV 2x2 Binned", nick = "wfov_2x2_binned")]
    Wfov2x2Binned,
    #[genum(name = "WFOV unbinned", nick = "wfov_unbinned")]
    WfovUnbinned,
}

/// Try to convert a `k4a::DepthMode` into a [K4aDepthMode](enum.K4aDepthMode.html). This is a
/// TryFrom, as `k4a::DepthMode` is wider than [K4aDepthMode](enum.K4aDepthMode.html).
impl TryFrom<k4a::DepthMode> for K4aDepthMode {
    type Error = K4aSrcError;

    fn try_from(dm: k4a::DepthMode) -> Result<Self, Self::Error> {
        match dm {
            DepthMode::K4A_DEPTH_MODE_NFOV_2X2BINNED => Ok(K4aDepthMode::Nfov2x2Binned),
            DepthMode::K4A_DEPTH_MODE_NFOV_UNBINNED => Ok(K4aDepthMode::NfovUnbinned),
            DepthMode::K4A_DEPTH_MODE_WFOV_2X2BINNED => Ok(K4aDepthMode::Wfov2x2Binned),
            DepthMode::K4A_DEPTH_MODE_WFOV_UNBINNED => Ok(K4aDepthMode::WfovUnbinned),
            _ => Err(K4aSrcError::Failure(
                "Unsupported k4a::DepthMode conversion",
            )),
        }
    }
}
/// Convert a [K4aDepthMode](enum.K4aDepthMode.html) into a `k4a::DepthMode`. This can be
/// converted directly, as all values represented by [K4aDepthMode](enum.K4aDepthMode.html) is
/// also represented by `k4a::DepthMode`.
impl From<K4aDepthMode> for k4a::DepthMode {
    fn from(value: K4aDepthMode) -> Self {
        match value {
            K4aDepthMode::Nfov2x2Binned => DepthMode::K4A_DEPTH_MODE_NFOV_2X2BINNED,
            K4aDepthMode::NfovUnbinned => DepthMode::K4A_DEPTH_MODE_NFOV_UNBINNED,
            K4aDepthMode::Wfov2x2Binned => DepthMode::K4A_DEPTH_MODE_WFOV_2X2BINNED,
            K4aDepthMode::WfovUnbinned => DepthMode::K4A_DEPTH_MODE_WFOV_UNBINNED,
        }
    }
}

/// The Azure Kinect does not have a floating framerate, as with other cameras. We therefore
/// represent it as an enum here. This enum is used to implement it as a GStreamer property.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, GEnum)]
#[repr(u32)]
#[genum(type_name = "GstK4aFramerate")]
pub enum K4aFramerate {
    #[genum(name = "5 FPS", nick = "5fps")]
    FPS5,
    #[genum(name = "15 FPS", nick = "15fps")]
    FPS15,
    #[genum(
        name = "30 FPS (not available for `depth-mode=wfov_unbinned` or `color-resolution=3072p`)",
        nick = "30fps"
    )]
    FPS30,
}

impl K4aFramerate {
    pub fn allowed_framerates() -> Vec<i32> {
        vec![5, 15, 30]
    }
}

/// Convert a [K4aFramerate](enum.K4aFramerate.html) into `k4a::Fps`.
impl From<K4aFramerate> for k4a::Fps {
    fn from(fr: K4aFramerate) -> Self {
        match fr {
            K4aFramerate::FPS5 => k4a::Fps::K4A_FRAMES_PER_SECOND_5,
            K4aFramerate::FPS15 => k4a::Fps::K4A_FRAMES_PER_SECOND_15,
            K4aFramerate::FPS30 => k4a::Fps::K4A_FRAMES_PER_SECOND_30,
        }
    }
}
