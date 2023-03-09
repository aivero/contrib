extern crate capnp;
pub mod rs_meta_capnp {
    #![allow(dead_code)]
    #![allow(clippy::redundant_field_names)]
    include!(concat!(env!("OUT_DIR"), "/rs_meta_capnp.rs"));
}

pub mod rs_meta_serialization {
    use super::rs_meta_capnp::rs_metadata;
    use capnp::message::Builder;
    use capnp::serialize_packed;
    use std::io::Error;

    pub(crate) fn capnp_serialize(metadata: rs2::metadata::Metadata) -> Result<Vec<u8>, Error> {
        let mut message = Builder::new_default();
        {
            let mut md = message.init_root::<rs_metadata::Builder>();

            md.set_frame_counter(metadata.frame_counter.unwrap_or(0));
            md.set_frame_timestamp(metadata.frame_timestamp.unwrap_or(0));
            md.set_sensor_timestamp(metadata.sensor_timestamp.unwrap_or(0));
            md.set_actual_exposure(metadata.actual_exposure.unwrap_or(0));
            md.set_gain_level(metadata.gain_level.unwrap_or(0));
            md.set_auto_exposure(metadata.auto_exposure.unwrap_or(0));
            md.set_white_balance(metadata.white_balance.unwrap_or(0));
            md.set_time_of_arrival(metadata.time_of_arrival.unwrap_or(0));
            md.set_temperature(metadata.temperature.unwrap_or(0));
            md.set_backend_timestamp(metadata.backend_timestamp.unwrap_or(0));
            md.set_actual_fps(metadata.actual_fps.unwrap_or(0));
            md.set_laser_power(metadata.laser_power.unwrap_or(0));
            md.set_laser_power_mode(metadata.laser_power_mode.unwrap_or(0));
            md.set_exposure_priority(metadata.exposure_priority.unwrap_or(0));
            md.set_exposure_roi_left(metadata.exposure_roi_left.unwrap_or(0));
            md.set_exposure_roi_right(metadata.exposure_roi_right.unwrap_or(0));
            md.set_exposure_roi_top(metadata.exposure_roi_top.unwrap_or(0));
            md.set_exposure_roi_bottom(metadata.exposure_roi_bottom.unwrap_or(0));
            md.set_brightness(metadata.brightness.unwrap_or(0));
            md.set_contrast(metadata.contrast.unwrap_or(0));
            md.set_saturation(metadata.saturation.unwrap_or(0));
            md.set_sharpness(metadata.sharpness.unwrap_or(0));
            md.set_auto_white_balance_temperature(
                metadata.auto_white_balance_temperature.unwrap_or(0),
            );
            md.set_backlight_compensation(metadata.backlight_compensation.unwrap_or(0));
            md.set_hue(metadata.hue.unwrap_or(0));
            md.set_gamma(metadata.gamma.unwrap_or(0));
            md.set_manual_white_balance(metadata.white_balance.unwrap_or(0));
            md.set_power_line_frequency(metadata.power_line_frequency.unwrap_or(0));
            md.set_low_light_compensation(metadata.low_light_compensation.unwrap_or(0));
        }

        let mut enc: Vec<u8> = Vec::new();
        serialize_packed::write_message(&mut enc, &message).unwrap();
        Ok(enc)
    }
}
