#[macro_use]
extern crate gst;
extern crate gst_base;
extern crate gst_depth_meta;

mod common;
mod rgbddemux;
mod rgbdmux;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), gst::glib::BoolError> {
    rgbddemux::register(plugin)?;
    rgbdmux::register(plugin)?;
    Ok(())
}

plugin_define!(
    rgbd,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    env!("CARGO_PKG_VERSION"),
    "MIT",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
