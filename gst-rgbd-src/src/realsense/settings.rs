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

use glib::*;
use std::fmt::{Display, Formatter};

pub(crate) use rs2::stream_profile::StreamResolution;

pub(crate) use super::streams::StreamId;

// Default behaviour of playing from rosbag recording specified by `rosbag-location` property.
pub(crate) const DEFAULT_LOOP_ROSBAG: bool = false;

// Default timeout used while waiting for frames from a realsense device in milliseconds.
pub(crate) const DEFAULT_PIPELINE_WAIT_FOR_FRAMES_TIMEOUT: u32 = 2500;

// Default behaviour for enablind metadata
pub(crate) const DEFAULT_ENABLE_METADATA: bool = false;

// Default behaviour for playing back from rosbag recording.
pub(crate) const DEFAULT_REAL_TIME_ROSBAG_PLAYBACK: bool = false;

/// Default behaviour for attaching camera meta buffers.
pub(crate) const DEFAULT_ATTACH_CAMERA_META: bool = false;

// Streams enabled by defaults
pub(crate) const DEFAULT_ENABLE_DEPTH: bool = true;
pub(crate) const DEFAULT_ENABLE_INFRA1: bool = false;
pub(crate) const DEFAULT_ENABLE_INFRA2: bool = false;
pub(crate) const DEFAULT_ENABLE_COLOR: bool = false;

// Default framerate
pub(crate) const DEFAULT_FRAMERATE: i32 = 30;

// Default resolution of depth, infra1 and infra2 streams
pub(crate) const DEFAULT_DEPTH_WIDTH: i32 = 1280;
pub(crate) const DEFAULT_DEPTH_HEIGHT: i32 = 720;

// Default resolution of color stream
pub(crate) const DEFAULT_COLOR_WIDTH: i32 = 1280;
pub(crate) const DEFAULT_COLOR_HEIGHT: i32 = 720;

/// A struct containing properties of `realsensesrc`
pub(crate) struct Settings {
    pub(crate) serial: Option<String>,
    pub(crate) rosbag_location: Option<String>,
    pub(crate) config: Option<String>,
    pub(crate) streams: StreamsSettings,
    pub(crate) loop_rosbag: bool,
    pub(crate) wait_for_frames_timeout: u32,
    pub(crate) include_per_frame_metadata: bool,
    pub(crate) real_time_rosbag_playback: bool,
    pub(crate) attach_camera_meta: bool,
    /// The stream identifier of the stream to align to.
    pub(crate) align_to: StreamId,
    pub(crate) log_level: LogLevel,
}

/// A struct containing properties of `realsensesrc` about streams
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StreamsSettings {
    pub(crate) enabled_streams: EnabledStreams,
    pub(crate) depth_resolution: StreamResolution,
    pub(crate) color_resolution: StreamResolution,
    pub(crate) framerate: i32,
}

impl Display for StreamsSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let depth_info = &format!("{}@{}fps", self.depth_resolution, self.framerate);
        let color_info = &format!("{}@{}fps", self.color_resolution, self.framerate);
        write!(
            f,
            "depth: {}, color: {}, infra1: {}, infra2: {}",
            if self.enabled_streams.depth {
                depth_info
            } else {
                "disabled"
            },
            if self.enabled_streams.color {
                color_info
            } else {
                "disabled"
            },
            if self.enabled_streams.infra1 {
                depth_info
            } else {
                "disabled"
            },
            if self.enabled_streams.infra2 {
                depth_info
            } else {
                "disabled"
            },
        )
    }
}

impl StreamsSettings {
    /// Get resolution of stream determined by `stream_id`.
    /// # Arguments
    /// * `stream_id` - Stream for which to return the resolution.
    /// # Returns
    /// `(i32, i32)` - Resolution of the stream formated as tuple=(width, height).
    pub(crate) fn get_stream_resolution(&self, stream_id: StreamId) -> (i32, i32) {
        // Depth, infra1 and infra2 streams share the same resolution.
        match stream_id {
            StreamId::Color => (self.color_resolution.width, self.color_resolution.height),
            _ => (self.depth_resolution.width, self.depth_resolution.height),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            rosbag_location: None,
            serial: None,
            config: None,
            streams: StreamsSettings {
                enabled_streams: EnabledStreams {
                    depth: DEFAULT_ENABLE_DEPTH,
                    infra1: DEFAULT_ENABLE_INFRA1,
                    infra2: DEFAULT_ENABLE_INFRA2,
                    color: DEFAULT_ENABLE_COLOR,
                },
                depth_resolution: StreamResolution {
                    width: DEFAULT_DEPTH_WIDTH,
                    height: DEFAULT_DEPTH_HEIGHT,
                },
                color_resolution: StreamResolution {
                    width: DEFAULT_COLOR_WIDTH,
                    height: DEFAULT_COLOR_HEIGHT,
                },
                framerate: DEFAULT_FRAMERATE,
            },
            loop_rosbag: DEFAULT_LOOP_ROSBAG,
            wait_for_frames_timeout: DEFAULT_PIPELINE_WAIT_FOR_FRAMES_TIMEOUT,
            include_per_frame_metadata: DEFAULT_ENABLE_METADATA,
            real_time_rosbag_playback: DEFAULT_REAL_TIME_ROSBAG_PLAYBACK,
            attach_camera_meta: DEFAULT_ATTACH_CAMERA_META,
            align_to: StreamId::default(),
            log_level: LogLevel::default(),
        }
    }
}

/// Helper struct that contains information about what streams are enabled
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EnabledStreams {
    /// Flag that determines if depth stream is enabled.
    pub(crate) depth: bool,
    /// Flag that determines if infra1 stream is enabled.
    pub(crate) infra1: bool,
    /// Flag that determines if infra2 stream is enabled.
    pub(crate) infra2: bool,
    /// Flag that determines if color stream is enabled.
    pub(crate) color: bool,
}

impl EnabledStreams {
    /// Determines whether at least one stream is enabled.
    ///
    /// # Returns
    /// * `true` if at least one stream is enabled.
    /// * `false` if no stream is enabled.
    pub(crate) fn any(&self) -> bool {
        self.depth || self.infra1 || self.infra2 || self.color
    }

    /// Determines whether there are any conflict between `enabled_streams` and
    /// `available_streams`
    ///
    /// # Arguments
    /// * `enabled_streams` - The streams that are enabled.
    /// * `available_streams` - The streams that are available.
    ///
    /// # Returns
    /// * `Vec<StreamId>` of conflicting streams, which is empty if there is no conflict.
    pub(crate) fn get_conflicts(&self, available_streams: &EnabledStreams) -> Vec<StreamId> {
        let mut conflicting_streams = Vec::new();
        if self.depth && !available_streams.depth {
            conflicting_streams.push(StreamId::Depth);
        }
        if self.infra1 && !available_streams.infra1 {
            conflicting_streams.push(StreamId::Infra1);
        }
        if self.infra2 && !available_streams.infra2 {
            conflicting_streams.push(StreamId::Infra2);
        }
        if self.color && !available_streams.color {
            conflicting_streams.push(StreamId::Color);
        }
        conflicting_streams
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Enum)]
#[repr(u32)]
#[enum_type(name = "GstRealsenseSrcLogLevel")]
pub(crate) enum LogLevel {
    #[enum_value(name = "Debug", nick = "debug")]
    Debug,
    #[enum_value(name = "Info", nick = "info")]
    Info,
    #[enum_value(name = "Warn", nick = "warn")]
    Warn,
    #[enum_value(name = "Error", nick = "error")]
    Error,
    #[enum_value(name = "Fatal", nick = "fatal")]
    Fatal,
    #[enum_value(name = "None", nick = "none")]
    None,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Error
    }
}

impl LogLevel {
    pub(crate) fn to_rs2_log_level(self) -> rs2::rs2_log_severity {
        match self {
            Self::Debug => rs2::rs2_log_severity::RS2_LOG_SEVERITY_DEBUG,
            Self::Info => rs2::rs2_log_severity::RS2_LOG_SEVERITY_INFO,
            Self::Warn => rs2::rs2_log_severity::RS2_LOG_SEVERITY_WARN,
            Self::Error => rs2::rs2_log_severity::RS2_LOG_SEVERITY_ERROR,
            Self::Fatal => rs2::rs2_log_severity::RS2_LOG_SEVERITY_FATAL,
            Self::None => rs2::rs2_log_severity::RS2_LOG_SEVERITY_NONE,
        }
    }
}
