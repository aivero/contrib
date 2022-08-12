// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.

use std::collections::HashMap;

pub enum MetadataAttribute {
    FrameCounter = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_COUNTER as isize,
    FrameTimestamp = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_TIMESTAMP as isize,
    SensorTimestamp = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SENSOR_TIMESTAMP as isize,
    ActualExposure = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_ACTUAL_EXPOSURE as isize,
    GainLevel = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_GAIN_LEVEL as isize,
    AutoExposure = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_AUTO_EXPOSURE as isize,
    WhiteBalance = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_WHITE_BALANCE as isize,
    TimeOfArrival = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_TIME_OF_ARRIVAL as isize,
    Temperature = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_TEMPERATURE as isize,
    BackendTimestamp = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_BACKEND_TIMESTAMP as isize,
    ActualFPS = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_ACTUAL_FPS as isize,
    LaserPower = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LASER_POWER as isize,
    LaserPowerMode =
        rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LASER_POWER_MODE as isize,
    ExposurePriority = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_PRIORITY as isize,
    ExposureRoiLeft = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_LEFT as isize,
    ExposureRoiRight = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_RIGHT as isize,
    ExposureRoiTop = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_TOP as isize,
    ExposureRoiBottom =
        rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_BOTTOM as isize,
    Brightness = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_BRIGHTNESS as isize,
    Contrast = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_CONTRAST as isize,
    Saturation = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SATURATION as isize,
    Sharpness = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SHARPNESS as isize,
    WhiteBalanceTemperature =
        rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_AUTO_WHITE_BALANCE_TEMPERATURE as isize,
    BacklightCompensation =
        rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_BACKLIGHT_COMPENSATION as isize,
    Hue = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_HUE as isize,
    Gamma = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_GAMMA as isize,
    ManualWhiteBalance =
        rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_MANUAL_WHITE_BALANCE as isize,
    PowerLineFrequency =
        rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_POWER_LINE_FREQUENCY as isize,
    LowLightCompensation =
        rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_LOW_LIGHT_COMPENSATION as isize,
    FrameEmitterMode = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_EMITTER_MODE as isize,
    FrameLedPower = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LED_POWER as isize,
    RawFrameSize = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_RAW_FRAME_SIZE as isize,
    GpioInputData = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_GPIO_INPUT_DATA as isize,
    SequenceName = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SEQUENCE_NAME as isize,
    SequenceId = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SEQUENCE_ID as isize,
    SequenceSize = rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SEQUENCE_SIZE as isize,
}

#[derive(Debug, Default)]
pub struct Metadata {
    pub frame_counter: Option<i64>,
    pub frame_timestamp: Option<i64>,
    pub sensor_timestamp: Option<i64>,
    pub actual_exposure: Option<i64>,
    pub gain_level: Option<i64>,
    pub auto_exposure: Option<i64>,
    pub white_balance: Option<i64>,
    pub time_of_arrival: Option<i64>,
    pub temperature: Option<i64>,
    pub backend_timestamp: Option<i64>,
    pub actual_fps: Option<i64>,
    pub laser_power: Option<i64>,
    pub laser_power_mode: Option<i64>,
    pub exposure_priority: Option<i64>,
    pub exposure_roi_left: Option<i64>,
    pub exposure_roi_right: Option<i64>,
    pub exposure_roi_top: Option<i64>,
    pub exposure_roi_bottom: Option<i64>,
    pub brightness: Option<i64>,
    pub contrast: Option<i64>,
    pub saturation: Option<i64>,
    pub sharpness: Option<i64>,
    pub auto_white_balance_temperature: Option<i64>,
    pub backlight_compensation: Option<i64>,
    pub hue: Option<i64>,
    pub gamma: Option<i64>,
    pub manual_white_balance: Option<i64>,
    pub power_line_frequency: Option<i64>,
    pub low_light_compensation: Option<i64>,
    pub frame_emitter_mode: Option<i64>,
    pub frame_led_power: Option<i64>,
    pub raw_frame_size: Option<i64>,
    pub gpio_input_data: Option<i64>,
    pub sequence_name: Option<i64>,
    pub sequence_id: Option<i64>,
    pub sequence_size: Option<i64>,
}

impl Metadata {
    pub(crate) fn from(values: HashMap<u32, i64>) -> Metadata {
        let mut md = Metadata::default();

        for (field_idnf, value) in values.iter() {
            match *field_idnf {
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_COUNTER => {
                    md.frame_counter = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_TIMESTAMP => {
                    md.frame_timestamp = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SENSOR_TIMESTAMP => {
                    md.sensor_timestamp = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_ACTUAL_EXPOSURE => {
                    md.actual_exposure = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_GAIN_LEVEL => {
                    md.gain_level = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_AUTO_EXPOSURE => {
                    md.auto_exposure = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_WHITE_BALANCE => {
                    md.white_balance = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_TIME_OF_ARRIVAL => {
                    md.time_of_arrival = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_TEMPERATURE => {
                    md.temperature = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_BACKEND_TIMESTAMP => {
                    md.backend_timestamp = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_ACTUAL_FPS => {
                    md.actual_fps = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LASER_POWER => {
                    md.laser_power = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LASER_POWER_MODE => {
                    md.laser_power_mode = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_PRIORITY => {
                    md.exposure_priority = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_LEFT => {
                    md.exposure_roi_left = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_RIGHT => {
                    md.exposure_roi_right = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_TOP => {
                    md.exposure_roi_top = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_EXPOSURE_ROI_BOTTOM => {
                    md.exposure_roi_bottom = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_BRIGHTNESS => {
                    md.brightness = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_CONTRAST => {
                    md.contrast = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SATURATION => {
                    md.saturation = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SHARPNESS => {
                    md.sharpness = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_AUTO_WHITE_BALANCE_TEMPERATURE => {
                    md.auto_white_balance_temperature = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_BACKLIGHT_COMPENSATION => {
                    md.backlight_compensation = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_HUE => {
                    md.hue = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_GAMMA => {
                    md.gamma = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_MANUAL_WHITE_BALANCE => {
                    md.manual_white_balance = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_POWER_LINE_FREQUENCY => {
                    md.power_line_frequency = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_LOW_LIGHT_COMPENSATION => {
                    md.low_light_compensation = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_EMITTER_MODE => {
                    md.frame_emitter_mode = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_FRAME_LED_POWER => {
                    md.frame_led_power = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_RAW_FRAME_SIZE => {
                    md.raw_frame_size = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_GPIO_INPUT_DATA => {
                    md.gpio_input_data = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SEQUENCE_NAME => {
                    md.sequence_name = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SEQUENCE_ID => {
                    md.sequence_id = Some(*value);
                }
                rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_SEQUENCE_SIZE => {
                    md.sequence_size = Some(*value);
                }
                _ => {}
            }
        }
        md
    }
}
