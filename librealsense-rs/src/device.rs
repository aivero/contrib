// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.
use crate::error::Error;
use crate::low_level_utils::cstring_to_string;
use crate::sensor::{Sensor, SensorList};

// Expose `rs2_camera_info` for external use.
pub use rs2::rs2_camera_info;

pub struct DeviceList {
    pub(crate) handle: *mut rs2::rs2_device_list,
}

impl Drop for DeviceList {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_delete_device_list(self.handle);
        }
    }
}

/// Struct representation of a [`Device`](../device/struct.Device.html) that wraps
/// around `rs2_device` handle, which exposes the functionality of RealSense devices.
pub struct Device {
    pub(crate) handle: *mut rs2::rs2_device,
}

/// Safe releasing of the `rs2_device` handle.
impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_delete_device(self.handle);
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    /// Create a static snapshot of all connected
    /// [`Sensor`](../sensor/struct.Sensor.html)s within a specific
    /// [`Device`](../device/struct.Device.html).
    ///
    /// # Returns
    /// * `Ok(Vec<Sensor>)` on success.
    /// * `Err(Error)` on failure.
    pub fn query_sensors(&self) -> Result<Vec<Sensor>, Error> {
        let mut error = Error::default();
        let sensor_list = SensorList {
            handle: unsafe { rs2::rs2_query_sensors(self.handle, error.inner()) },
        };
        error.check()?;

        let mut error = Error::default();
        let sensor_count = unsafe { rs2::rs2_get_sensors_count(sensor_list.handle, error.inner()) };
        error.check()?;

        let mut sensors: Vec<Sensor> = Vec::new();
        sensors.reserve_exact(sensor_count as usize);

        for sensor_index in 0..sensor_count {
            let mut error = Error::default();
            sensors.push(Sensor {
                handle: unsafe {
                    rs2::rs2_create_sensor(sensor_list.handle, sensor_index, error.inner())
                },
            });
            error.check()?;
        }
        Ok(sensors)
    }

    #[deprecated(
        since = "0.6.0",
        note = "Use `query_sensors()` to be consistent with C/C++ API"
    )]
    pub fn get_sensors(&self) -> Result<Vec<Sensor>, Error> {
        self.query_sensors()
    }

    /// Check if a specific camera `info` is supported by the
    /// [`Device`](../device/struct.Device.html).
    ///
    /// # Arguments
    /// * `info` - The parameter to check for support.
    ///
    /// # Returns
    /// * `Ok(bool)` on success.
    /// * `Err(Error)` on failure.
    pub fn supports_info(&self, _info: rs2_camera_info) -> Result<bool, Error> {
        unimplemented!()
    }

    /// Retrieve camera specific information, like versions of various internal components.
    ///
    /// # Arguments
    /// * `info` - The camera info type to retrieve. Please see
    /// [rs2_camera_info](../device/enum.rs2_camera_info.html) for more information.
    ///
    /// # Returns
    /// * `Ok(String)` on success, containing the value under the info field.
    /// * `Err(Error)` on failure.
    pub fn get_info(&self, info: rs2_camera_info) -> Result<String, Error> {
        let mut error = Error::default();
        let ret = unsafe { rs2::rs2_get_device_info(self.handle, info, error.inner()) };
        error.check()?;
        Ok(cstring_to_string(ret))
    }

    /// Send hardware reset request to the [`Device`](../device/struct.Device.html).
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn hardware_reset(&self) -> Result<(), Error> {
        let mut error = Error::default();
        unsafe {
            rs2::rs2_hardware_reset(self.handle, error.inner());
        }
        error.check()?;
        Ok(())
    }

    /// Update [`Device`](../device/struct.Device.html) to the provided firmware, the
    /// device must be extendable to `RS2_EXTENSION_UPDATABLE`. This call is executed on the
    /// caller's thread and it supports progress notifications via the optional callback.
    ///
    /// # Arguments
    /// * `info` - The parameter to check for support.
    /// * `fw_image` - Firmware image buffer.
    /// * `fw_image_size` - Firmware image buffer size.
    /// * `callback` - Optional callback for update progress notifications, the progress value is
    /// normailzed to 1.
    /// * `client_data` - Optional client data for the callback.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn update_firmware(&self) -> Result<(), Error> {
        unimplemented!();
    }

    /// Send hardware reset request to the [`Device`](../device/struct.Device.html).
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn is_advanced_mode_enabled(&self) -> Result<bool, Error> {
        let mut error = Error::default();
        let is_enabled: &mut i32 = &mut (-1);
        unsafe {
            rs2::rs2_is_enabled(self.handle, is_enabled as *mut i32, error.inner());
        }
        error.check()?;
        Ok(*is_enabled == 1)
    }

    /// Enable or disable advanced mode for a [`Device`](../device/struct.Device.html).
    ///
    /// # Arguments
    /// * `enable` - The desired state of advanced mode after callback.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn set_advanced_mode(&self, enable: bool) -> Result<(), Error> {
        let mut error = Error::default();
        unsafe {
            rs2::rs2_toggle_advanced_mode(self.handle, enable as i32, error.inner());
        }
        error.check()?;
        Ok(())
    }

    /// Configure device with JSON.
    ///
    /// # Arguments
    /// * `json_content` - The content of the JSON configuration.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn load_json(&self, json_content: &str) -> Result<(), Error> {
        let mut error = Error::default();

        let s = std::ffi::CString::new(json_content).expect("Failed to create CString");
        unsafe {
            rs2::rs2_load_json(
                self.handle,
                s.as_ptr() as *const std::os::raw::c_void,
                json_content.len() as u32,
                error.inner(),
            );
        };

        error.check()?;
        Ok(())
    }

    /// Configure [`Device`](../device/struct.Device.html) with JSON file specified by
    /// `json_path`.
    ///
    /// # Arguments
    /// * `json_path` - The absolute path to JSON file.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn load_json_file_path(&self, json_path: &str) -> Result<(), Error> {
        if !self.is_advanced_mode_enabled()? {
            self.set_advanced_mode(true)?;
        }
        let json_content = std::fs::read_to_string(json_path).map_err(|err| {
            Error::new(
                &format!(
                    "Cannot read RealSense JSON configuration from file \"{}\" - {}",
                    json_path, err
                ),
                "Device::load_json_file_path()",
                "json_path",
                0,
            )
        })?;
        self.load_json(&json_content)?;
        Ok(())
    }

    /// Set the [`Playback`](../record_playback/struct.Playback.html) to work in real time or non
    /// real time. In real time mode, [`Playback`](../record_playback/struct.Playback.html) will
    /// play the same way the file was recorded. In real time mode if the application takes too
    /// long to handle the callback, frames may be dropped. In non real time mode,
    /// [`Playback`](../record_playback/struct.Playback.html) will wait for each callback to finish
    /// handling the data before reading the next frame. In this mode no frames will be dropped,
    /// and the application controls the frame rate of the
    /// [`Playback`](../record_playback/struct.Playback.html) (according to the callback handler
    /// duration).
    ///
    /// # Arguments
    /// * `enable` - Set `true` for real time mode and `false` for non real time mode.
    ///
    /// connected device.
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn set_real_time(&self, enable: bool) -> Result<(), Error> {
        let mut error = Error::default();
        unsafe { rs2::rs2_playback_device_set_real_time(self.handle, enable as i32, error.inner()) }
        error.check()?;
        Ok(())
    }

    /// Indicates if playback is in real time mode or non real time.
    ///
    /// connected device.
    /// # Returns
    /// * `Ok(bool)` on success, `true` for real time mode and `false` for non real time mode.
    /// * `Err(Error)` on failure.
    pub fn is_real_time(&self) -> Result<bool, Error> {
        let mut error = Error::default();
        let ret = unsafe { rs2::rs2_playback_device_is_real_time(self.handle, error.inner()) };
        error.check()?;
        Ok(ret != 0)
    }
}
