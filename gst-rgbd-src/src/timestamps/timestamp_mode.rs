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
use gst::glib;

/// Timestamp mode, which is used to determine the timestamps of outgoing buffers.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Enum)]
#[repr(u32)]
#[enum_type(name = "GstTimestampMode")]
pub enum TimestampMode {
    /// Don't do any timestamping.
    #[enum_value(name = "Ignore: Do not apply timestamp to any buffer", nick = "ignore")]
    Ignore = 0,
    /// Timestamp only the main buffer based on running time.
    #[enum_value(
        name = "ClockMain: Determine timestamps based on the current running time. Apply timestamps only to buffers of the main stream. Note that functionality of this variant is identical to enabling `do-timestamp` property.",
        nick = "clock_main"
    )]
    ClockMain = 1,
    /// Timestamp all buffers based on running time.
    #[enum_value(
        name = "ClockAll: Determine timestamps based on the current running time. Apply timestamps to buffers of all streams.",
        nick = "clock_all"
    )]
    ClockAll = 2,
    /// Timestamp all buffers based on camera timestamps. All buffers in a single frameset have the same timestamp.
    #[enum_value(
        name = "CameraCommon: Utilise timestamps acquired from camera or recording, if applicable. Apply timestamps to buffers of all streams. A common timestamp is applied to all buffers that belong to a single frameset, where the timestamp for each frameset is based on a frame that belongs to the main stream.",
        nick = "camera_common"
    )]
    CameraCommon = 3,
    /// Timestamp all buffers based on camera timestamps. Buffers in a single frameset can have different timestamps.
    #[enum_value(
        name = "CameraIndividual: Utilise timestamps acquired from camera or recording, if applicable. Apply timestamps to buffers of all streams. Individual timestamps are applied to buffers that belong to a single frameset, where the timestamp of each buffer is based on corresponding frame. Note that certain cameras can have some of all of the streams synchronised.",
        nick = "camera_individual"
    )]
    CameraIndividual = 4,
    /// Timestamp all buffers based on counting frames and the negotiated framerate.
    #[enum_value(
        name = "FrameCounting: Generate timestamps by counting frames and the negotiated framerate. Apply timestamps to buffers of all streams. A common timestamp is applied to all buffers that belong to a single frameset.",
        nick = "frame_counting"
    )]
    FrameCounting = 5,
}

/// Implentation of Default trait for TimestampMode, which returns `TimestampMode::CameraCommon`.
impl Default for TimestampMode {
    fn default() -> Self {
        TimestampMode::CameraCommon
    }
}

impl TimestampMode {
    /// Return `timestamp-mode` property definition that can be utilised by elements that use `TimestampMode` struct.
    /// Element utilising this property also needs to implement the corresponding variants for .property()` and `set_property()`
    pub fn get_property_type() -> glib::ParamSpec {
        glib::ParamSpecEnum::new(
                "timestamp-mode",
                "Timestamp Mode",
                "This property determines what timestamp mode to use for the outgoing `video/rgbd` stream. If implemented, please ignore `do-timestamp` property.",
                TimestampMode::static_type(),
                TimestampMode::default() as i32,
                glib::ParamFlags::READWRITE,
            )
    }
}
