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

#[macro_use]
extern crate gst;
extern crate gst_base;
extern crate gst_depth_meta;
extern crate gst_video;
#[macro_use]
extern crate lazy_static;
use std::sync::Once;

#[cfg(feature = "librealsense2")]
extern crate librealsense2 as rs2;

#[cfg(feature = "libk4a")]
mod k4a;
#[cfg(feature = "librealsense2")]
mod realsense;
mod timestamps;

static TAGS: Once = Once::new();

fn plugin_init(plugin: &gst::Plugin) -> Result<(), gst::glib::BoolError> {
    #[cfg(feature = "libk4a")]
    k4a::k4asrc::register(plugin)?;
    #[cfg(feature = "librealsense2")]
    realsense::realsensesrc::register(plugin)?;

    TAGS.call_once(|| {
        gst::tags::register::<gst_depth_meta::camera_meta::CameraMetaTag>();
    });

    let _ = plugin;
    Ok(())
}

plugin_define!(
    rgbdsrc,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    env!("CARGO_PKG_VERSION"),
    "LGPL",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
