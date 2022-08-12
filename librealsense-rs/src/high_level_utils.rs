// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.
use crate::config::Config;
use crate::context::Context;
use crate::error::Error;
use crate::pipeline::Pipeline;
use crate::pipeline_profile::PipelineProfile;
use crate::sensor::Sensor;
use crate::stream_profile::{StreamData, StreamResolution};

use rs2::rs2_camera_info::*;
use rs2::rs2_l500_visual_preset::*;
use rs2::rs2_option::*;
use rs2::rs2_rs400_visual_preset::*;
use rs2::rs2_sr300_visual_preset::*;

/// Print to STDOUT what RealSense [`Device`](../device/struct.Device.html)s are connected.
///
/// # Returns
/// * `Ok()` on success.
/// * `Err(Error)` on failure.
pub fn list_connected_devices() -> Result<(), Error> {
    let context = Context::create()?;
    let devices = context.query_devices()?;
    let device_count = devices.count()?;
    println!("-------------------------");
    if device_count == 0 {
        println!("No RealSense device is connected.");
    } else {
        println!(
            "The following {} RealSense devices are connected:",
            device_count
        );
        for i in 0..device_count {
            let device = devices.create_device(i)?;
            let name = device.get_info(RS2_CAMERA_INFO_NAME)?;
            let serial = device.get_info(RS2_CAMERA_INFO_SERIAL_NUMBER)?;
            let version = device.get_info(RS2_CAMERA_INFO_FIRMWARE_VERSION)?;
            let port = device.get_info(RS2_CAMERA_INFO_PHYSICAL_PORT)?;
            println!(
                "{}\tport:{}\tserial:{}\tversion:{}",
                name, port, serial, version
            );
        }
    }
    println!("-------------------------");
    Ok(())
}

/// Start a connected [`Device`](../device/struct.Device.html) with the corresponding
/// [`Config`](../config/struct.Config.html) and `index`.
///
/// # Arguments
/// * `config` - A [`Config`](../config/struct.Config.html) with
/// requested filters on the [`Pipeline`](../pipeline/struct.Pipeline.html) configuration.
/// * `index` - An index of the [`Device`](../device/struct.Device.html). Set to 0 to enable first
/// connected device.
///
/// # Returns
/// * `Ok(Pipeline)` on success.
/// * `Err(Error)` on failure.
pub fn start_device_with_index(config: &mut Config, index: i32) -> Result<Pipeline, Error> {
    let context = Context::create()?;
    let devices = context.query_devices()?;
    let device_count = devices.count()?;
    if index >= device_count {
        Err(Error::default())
    } else {
        let device = devices.create_device(index)?;
        let serial = device.get_info(RS2_CAMERA_INFO_SERIAL_NUMBER)?;
        config.enable_device(&serial)?;
        let pipeline = Pipeline::create(&context)?;
        pipeline.start_with_config(config)?;
        Ok(pipeline)
    }
}

/// Helper struct for `get_info_all_streams()`.
pub struct StreamInfo {
    pub data: StreamData,
    pub resolution: StreamResolution,
}
/// Retrieve information about all enabled streams based on a running
/// [`Pipeline`](../pipeline/struct.Pipeline.html).
///
/// # Arguments
/// * pipeline_profile - The [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html)
/// to extract the information from.
///
/// # Returns
/// * `Ok(Vec<StreamInfo>)` on success.
/// * `Err(Error)` on failure.
pub fn get_info_all_streams(pipeline_profile: &PipelineProfile) -> Result<Vec<StreamInfo>, Error> {
    let mut info_all_streams: Vec<StreamInfo> = Vec::new();
    let streams = pipeline_profile.get_streams()?;
    for i in 0..streams.count()? {
        let stream_profile = streams.get(i)?;
        let stream_data = stream_profile.get_data()?;
        let stream_resolution = stream_profile.get_resolution()?;
        info_all_streams.push(StreamInfo {
            data: stream_data,
            resolution: stream_resolution,
        })
    }
    Ok(info_all_streams)
}

/// Sets default visual preset on all depth sensors of each camera.
///
/// # Arguments
/// * pipeline_profile - The [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html)
/// to extract the information from.
///
/// # Returns
/// * `Ok(Vec<StreamInfo>)` on success.
/// * `Err(Error)` on failure.
pub fn set_default_visual_preset(sensor: &mut Sensor) -> Result<(), Error> {
    let default_preset_id = match sensor.get_info(RS2_CAMERA_INFO_PRODUCT_LINE)?.as_str() {
        "SR300" => RS2_SR300_VISUAL_PRESET_DEFAULT as u32,
        "D400" => RS2_RS400_VISUAL_PRESET_DEFAULT as u32,
        "L500" => RS2_L500_VISUAL_PRESET_DEFAULT as u32,
        product_line => {
            return Err(Error::new(
                &format!("Product line \"{}\" is unknown.", product_line),
                "Sensor::set_default_visual_preset()",
                "",
                0,
            ));
        }
    };
    sensor.set_option(RS2_OPTION_VISUAL_PRESET, default_preset_id as f32)
}
