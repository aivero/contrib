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

use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use super::settings::EnabledStreams;

/// ID/tag of the depth stream.
const STREAM_ID_DEPTH: &str = "depth";
/// ID/tag of the infra1 stream.
const STREAM_ID_INFRA1: &str = "infra1";
/// ID/tag of the infra2 stream.
const STREAM_ID_INFRA2: &str = "infra2";
/// ID/tag of the color stream.
const STREAM_ID_COLOR: &str = "color";
/// ID/tag of the camera meta stream.
/// TODO: This stream is not specific to RealSense, move it to the base class once we work on it.
pub(crate) const STREAM_ID_CAMERAMETA: &str = "camerameta";

/// Vec that contains list of enabled streams together with their corresponding stream description.
pub(crate) type Streams = Vec<(StreamId, StreamDescriptor)>;

/// Unique identified of each RealSense stream that is currently supported.
/// Note that this enum contains conversions to many useful types.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum StreamId {
    /// Depth stream.
    Depth,
    /// Infra1 stream.
    Infra1,
    /// Infra2 stream.
    Infra2,
    /// Color stream.
    Color,
}

impl Display for StreamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StreamId::Depth => STREAM_ID_DEPTH,
                StreamId::Infra1 => STREAM_ID_INFRA1,
                StreamId::Infra2 => STREAM_ID_INFRA2,
                StreamId::Color => STREAM_ID_COLOR,
            }
        )
    }
}

impl TryFrom<&str> for StreamId {
    type Error = gst::ErrorMessage;

    fn try_from(stream: &str) -> Result<Self, Self::Error> {
        match stream {
            "depth" => Ok(Self::Depth),
            "color" => Ok(Self::Color),
            "infra1" => Ok(Self::Infra1),
            "infra2" => Ok(Self::Infra2),
            _ => Err(gst_error_msg!(
                gst::StreamError::Failed,
                ["{} is not a valid stream", stream]
            )),
        }
    }
}

impl TryFrom<String> for StreamId {
    type Error = gst::ErrorMessage;

    fn try_from(stream: String) -> Result<Self, Self::Error> {
        StreamId::try_from(stream.as_str())
    }
}

impl From<StreamId> for RsStreamDescriptor {
    fn from(id: StreamId) -> RsStreamDescriptor {
        match id {
            StreamId::Depth => RsStreamDescriptor::new(
                rs2::rs2_stream::RS2_STREAM_DEPTH,
                rs2::rs2_format::RS2_FORMAT_Z16,
                -1,
            ),
            StreamId::Infra1 => RsStreamDescriptor::new(
                rs2::rs2_stream::RS2_STREAM_INFRARED,
                rs2::rs2_format::RS2_FORMAT_Y8,
                1,
            ),
            StreamId::Infra2 => RsStreamDescriptor::new(
                rs2::rs2_stream::RS2_STREAM_INFRARED,
                rs2::rs2_format::RS2_FORMAT_Y8,
                2,
            ),
            StreamId::Color => RsStreamDescriptor::new(
                rs2::rs2_stream::RS2_STREAM_COLOR,
                rs2::rs2_format::RS2_FORMAT_RGB8,
                -1,
            ),
        }
    }
}

impl From<RsStreamDescriptor> for StreamId {
    fn from(rs_stream_descriptor: RsStreamDescriptor) -> Self {
        match rs_stream_descriptor.rs2_stream {
            rs2::rs2_stream::RS2_STREAM_DEPTH => match rs_stream_descriptor.sensor_id {
                i if i<1 => StreamId::Depth,
                _ => unreachable!("Each RealSense device has only one depth stream, the selected stream index of {} is invalid", rs_stream_descriptor.sensor_id)
            }
            rs2::rs2_stream::RS2_STREAM_INFRARED => match rs_stream_descriptor.sensor_id {
                1 => StreamId::Infra1,
                2 => StreamId::Infra2,
                _ => unreachable!("Each RealSense device has only two infrared streams"),
            },
            rs2::rs2_stream::RS2_STREAM_COLOR =>  match rs_stream_descriptor.sensor_id {
                i if i<1 => StreamId::Color,
                _ => unreachable!("Each RealSense device has only one color stream, the selected stream index of {} is invalid", rs_stream_descriptor.sensor_id)
            }
            _ => unimplemented!("Other RealSense streams are not supported"),
        }
    }
}

impl From<StreamId> for gst_video::VideoFormat {
    fn from(id: StreamId) -> gst_video::VideoFormat {
        match id {
            StreamId::Depth => gst_video::VideoFormat::Gray16Le,
            StreamId::Infra1 | StreamId::Infra2 => gst_video::VideoFormat::Gray8,
            StreamId::Color => gst_video::VideoFormat::Rgb,
        }
    }
}

impl From<StreamId> for StreamDescriptor {
    fn from(id: StreamId) -> StreamDescriptor {
        StreamDescriptor::new(id.into(), id.into())
    }
}

/// Struct that contains unique description of RealSense stream, which can be used to identify
/// the corresponding stream using solely librealsense API.
pub(crate) struct RsStreamDescriptor {
    /// Unique identifier of the RealSense stream type.
    pub(crate) rs2_stream: rs2::rs2_stream,
    /// RealSense-specific format of the stream.
    pub(crate) rs2_format: rs2::rs2_format,
    /// Index of the sensor from which the stream is produced. This field is relevant only if
    /// camera produces multiple streams of the same type (in stereo setup).
    pub(crate) sensor_id: i32,
}

impl RsStreamDescriptor {
    pub fn new(rs2_stream: rs2::rs2_stream, rs2_format: rs2::rs2_format, sensor_id: i32) -> Self {
        Self {
            rs2_stream,
            rs2_format,
            sensor_id,
        }
    }
}

/// Struct that contains unique description of stream in the context of this element. This struct contains
/// both GStreamer and RealSense-specific fields.
pub(crate) struct StreamDescriptor {
    /// Unique description of RealSense stream.
    pub(crate) rs2_stream_descriptor: RsStreamDescriptor,
    /// Video format of the stream.
    pub(crate) video_format: gst_video::VideoFormat,
}

impl StreamDescriptor {
    pub(crate) fn new(
        rs2_stream_descriptor: RsStreamDescriptor,
        video_format: gst_video::VideoFormat,
    ) -> Self {
        Self {
            rs2_stream_descriptor,
            video_format,
        }
    }
}

impl From<&EnabledStreams> for Streams {
    fn from(enabled_streams: &EnabledStreams) -> Self {
        let mut streams: Streams = Vec::new();
        if enabled_streams.depth {
            streams.push((StreamId::Depth, StreamId::Depth.into()));
        }
        if enabled_streams.infra1 {
            streams.push((StreamId::Infra1, StreamId::Infra1.into()));
        }
        if enabled_streams.infra2 {
            streams.push((StreamId::Infra2, StreamId::Infra2.into()));
        }
        if enabled_streams.color {
            streams.push((StreamId::Color, StreamId::Color.into()));
        }
        streams
    }
}
