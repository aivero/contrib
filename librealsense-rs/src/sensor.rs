// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.

use crate::error::Error;
use crate::low_level_utils::cstring_to_string;
use crate::stream_profile::*;
use rs2::rs2_camera_info;
use rs2::rs2_option;
use rs2::rs2_options;

pub struct SensorList(pub(crate) *mut rs2::rs2_sensor_list);

impl Drop for SensorList {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_sensor_list(self.0) }
    }
}

impl From<*mut rs2::rs2_sensor_list> for SensorList {
    fn from(s: *mut rs2::rs2_sensor_list) -> Self {
        SensorList(s)
    }
}

impl SensorList {
    /// Determines number of sensors in a list
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn count(&self) -> Result<i32, Error> {
        Error::call1(rs2::rs2_get_sensors_count, self.0)
    }

    /// Create sensor by index
    ///
    /// # Arguments
    /// * `index` - The zero based index of sensor to retrieve
    ///
    /// # Returns
    /// * `Ok(Sensor)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_sensor(&self, i: i32) -> Result<Sensor, Error> {
        Error::call2(rs2::rs2_create_sensor, self.0, i)
    }
}

/// Struct representation of [`Sensor`](../sensor/struct.Sensor.html) that wraps around
/// `rs2_sensor` handle.
pub struct Sensor(pub(crate) *mut rs2::rs2_sensor);

/// Safe releasing of the `rs2_sensor` handle.
impl Drop for Sensor {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_sensor(self.0) }
    }
}

impl From<*mut rs2::rs2_sensor> for Sensor {
    fn from(s: *mut rs2::rs2_sensor) -> Self {
        Sensor(s)
    }
}

impl Sensor {
    /// Retrieve the [`StreamProfile`](../stream_profile/struct.StreamProfile.html)s of a
    /// [`Sensor`](../sensor/struct.Sensor.html).
    ///
    /// # Returns
    /// * `Ok(StreamProfileList)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_stream_profiles(&self) -> Result<StreamProfileList, Error> {
        Error::call1(rs2::rs2_get_stream_profiles, self.0)
    }

    /// When called on a depth [`Sensor`](../sensor/struct.Sensor.html), this method will return
    /// the number of meters represented by a single depth unit
    ///
    /// # Returns
    /// * `Ok(f32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_depth_scale(&self) -> Result<f32, Error> {
        Error::call1(rs2::rs2_get_depth_scale, self.0)
    }

    /// Retrieve sensor specific information, like versions of various internal components
    ///
    /// # Arguments
    /// * `info` - Camera info type to retrieve
    ///
    /// # Returns
    /// * `Ok(String)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_info(&self, info: rs2_camera_info) -> Result<String, Error> {
        let value = Error::call2(rs2::rs2_get_sensor_info, self.0, info)?;
        Ok(cstring_to_string(value))
    }

    /// Check if particular option is supported by a subdevice
    ///
    /// # Arguments
    /// * `option` - Option id to be checked
    ///
    /// # Returns
    /// * `Ok(bool)` on success.
    /// * `Err(Error)` on failure.
    pub fn supports_option(&self, option: rs2_option) -> Result<bool, Error> {
        Error::call2(
            rs2::rs2_supports_option,
            self.0.cast::<rs2_options>(),
            option,
        )
        .map(|i: i32| i != 0)
    }

    /// Check if an option is read-only
    ///
    /// # Arguments
    /// * `option` - Option id to be checked
    ///
    /// # Returns
    /// * `Ok(bool)` on success.
    /// * `Err(Error)` on failure.
    pub fn is_option_read_only(&self, option: rs2_option) -> Result<bool, Error> {
        Error::call2(
            rs2::rs2_is_option_read_only,
            self.0.cast::<rs2_options>(),
            option,
        )
        .map(|i: i32| i != 0)
    }

    /// Read option value from the sensor
    ///
    /// # Arguments
    /// * `option` - Option id to be queried
    ///
    /// # Returns
    /// * `Ok(f32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_option(&self, option: rs2_option) -> Result<f32, Error> {
        if !self.supports_option(option)? {
            return Err(Error::new(
                &format!(
                    "Cannot get RealSense option \"{:#?}\" because Sensor does not support it.",
                    option
                ),
                "Sensor::get_option()",
                "option",
                0,
            ));
        }

        Error::call2(
            rs2::rs2_get_option,
            self.0.cast::<rs2_options>(),
            rs2_option::RS2_OPTION_VISUAL_PRESET,
        )
    }

    /// Write new value to sensor option
    ///
    /// # Arguments
    /// * `option` - Option id to be queried
    /// * `value` - New value for the option
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn set_option(&mut self, option: rs2_option, value: f32) -> Result<(), Error> {
        if !self.supports_option(option)? {
            return Err(Error::new(
                &format!(
                    "Cannot set RealSense option \"{:#?}\" because Sensor does not support it.",
                    option
                ),
                "Sensor::set_option()",
                "option",
                0,
            ));
        }
        if self.is_option_read_only(option)? {
            return Err(Error::new(
                &format!(
                    "Cannot set RealSense option \"{:#?}\" because it is read-only.",
                    option
                ),
                "Sensor::set_option()",
                "option",
                0,
            ));
        }

        Error::call3(
            rs2::rs2_set_option,
            self.0.cast::<rs2_options>(),
            rs2_option::RS2_OPTION_VISUAL_PRESET,
            value as f32,
        )
    }
}
