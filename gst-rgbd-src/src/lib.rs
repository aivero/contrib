#[macro_use]
extern crate gst;
extern crate gst_base;
extern crate gst_depth_meta;
extern crate gst_video;
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
    "MIT",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
