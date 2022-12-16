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

#[macro_use]
extern crate gst;
extern crate gst_base;
extern crate gst_depth_meta;
extern crate gst_video;
extern crate nalgebra as na;

mod common;
mod framealigner;

use gst::glib;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    framealigner::register(plugin)?;
    Ok(())
}

plugin_define!(
    framealigner,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    env!("CARGO_PKG_VERSION"),
    "Proprietary",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    "2017-12-01"
);
