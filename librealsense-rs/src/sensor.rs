// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.

use crate::error::Error;
use crate::low_level_utils::cstring_to_string;
use crate::stream_profile::*;
use rs2::rs2_camera_info;
use rs2::rs2_option;
use rs2::rs2_options;

/// Struct representation of [`Sensor`](../sensor/struct.Sensor.html) that wraps around
/// `rs2_sensor` handle.
pub struct Sensor {
    pub(crate) handle: *mut rs2::rs2_sensor,
}

/// Safe releasing of the `rs2_sensor` handle.
impl Drop for Sensor {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_delete_sensor(self.handle);
        }
    }
}

pub struct SensorList {
    pub(crate) handle: *mut rs2::rs2_sensor_list,
}

impl Drop for SensorList {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_delete_sensor_list(self.handle);
        }
    }
}

impl Sensor {
    /// Retrieve the [`StreamProfile`](../stream_profile/struct.StreamProfile.html)s of a
    /// [`Sensor`](../sensor/struct.Sensor.html).
    ///
    /// # Returns
    /// * `Ok(Vec<StreamProfile>)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_stream_profiles(&self) -> Result<Vec<StreamProfile>, Error> {
        let mut error = Error::default();
        let profile_list = StreamProfileList {
            handle: unsafe { rs2::rs2_get_stream_profiles(self.handle, error.inner()) },
        };
        error.check()?;

        let mut error = Error::default();
        let count =
            unsafe { rs2::rs2_get_stream_profiles_count(profile_list.handle, error.inner()) };

        let mut res: Vec<StreamProfile> = Vec::new();
        for i in 0..count {
            let mut error = Error::default();
            res.push(StreamProfile {
                handle: unsafe {
                    rs2::rs2_get_stream_profile(profile_list.handle, i, error.inner())
                        as *mut rs2::rs2_stream_profile
                },
            });
            error.check()?;
        }
        Ok(res)
    }

    /// When called on a depth [`Sensor`](../sensor/struct.Sensor.html), this method will return
    /// the number of meters represented by a single depth unit
    ///
    /// # Returns
    /// * `Ok(f32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_depth_scale(&self) -> Result<f32, Error> {
        let mut error = Error::default();
        let depth_scale = unsafe { rs2::rs2_get_depth_scale(self.handle, error.inner()) };
        error.check()?;
        Ok(depth_scale)
    }

    pub fn get_info(&self, info: rs2_camera_info) -> Result<String, Error> {
        let mut error = Error::default();
        let value = unsafe { rs2::rs2_get_sensor_info(self.handle, info, error.inner()) };
        error.check()?;
        Ok(cstring_to_string(value))
    }

    pub fn supports_option(&self, option: rs2_option) -> Result<bool, Error> {
        let mut error = Error::default();
        let is_supported = unsafe {
            rs2::rs2_supports_option(self.handle.cast::<rs2_options>(), option, error.inner())
        };
        error.check()?;
        Ok(is_supported != 0)
    }

    pub fn is_option_read_only(&self, option: rs2_option) -> Result<bool, Error> {
        let mut error = Error::default();
        let is_read_only = unsafe {
            rs2::rs2_is_option_read_only(self.handle.cast::<rs2_options>(), option, error.inner())
        };
        error.check()?;
        Ok(is_read_only != 0)
    }

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

        let mut error = Error::default();
        let ret = unsafe {
            rs2::rs2_get_option(self.handle.cast::<rs2_options>(), option, error.inner())
        };
        error.check()?;
        Ok(ret)
    }

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

        let mut error = Error::default();
        unsafe {
            rs2::rs2_set_option(
                self.handle.cast::<rs2_options>(),
                rs2_option::RS2_OPTION_VISUAL_PRESET,
                value as f32,
                error.inner(),
            );
        }
        error.check()?;
        Ok(())
    }

    // unimplemented!
}
