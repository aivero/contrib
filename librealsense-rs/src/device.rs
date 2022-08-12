// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.
use crate::error::Error;
use crate::low_level_utils::cstring_to_string;
use crate::sensor::SensorList;

// Expose `rs2_camera_info` for external use.
pub use rs2::rs2_camera_info;

pub struct DeviceList(pub(crate) *mut rs2::rs2_device_list);

impl Drop for DeviceList {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_device_list(self.0) }
    }
}

impl From<*mut rs2::rs2_device_list> for DeviceList {
    fn from(d: *mut rs2::rs2_device_list) -> Self {
        DeviceList(d)
    }
}

impl DeviceList {
    /// Determines number of devices in a list.
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn count(&self) -> Result<i32, Error> {
        Error::call1(rs2::rs2_get_device_count, self.0)
    }

    /// Creates a device by index. The device object represents a physical camera and provides
    /// the means to manipulate it.
    ///
    /// # Arguments
    /// * `index` - The zero based index of device to retrieve.
    ///
    /// # Returns
    /// * `Ok(Device)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_device(&self, index: i32) -> Result<Device, Error> {
        Error::call2(rs2::rs2_create_device, self.0, index)
    }
}

/// Struct representation of a [`Device`](../device/struct.Device.html) that wraps
/// around `rs2_device` handle, which exposes the functionality of RealSense devices.
pub struct Device(pub(crate) *mut rs2::rs2_device);

/// Safe releasing of the `rs2_device` handle.
impl Drop for Device {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_device(self.0) }
    }
}

impl From<*mut rs2::rs2_device> for Device {
    fn from(d: *mut rs2::rs2_device) -> Self {
        Device(d)
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
    /// * `Ok(SensorList)` on success.
    /// * `Err(Error)` on failure.
    pub fn query_sensors(&self) -> Result<SensorList, Error> {
        Error::call1(rs2::rs2_query_sensors, self.0)
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
        let ret = Error::call2(rs2::rs2_get_device_info, self.0, info)?;
        Ok(cstring_to_string(ret))
    }

    /// Send hardware reset request to the [`Device`](../device/struct.Device.html).
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn hardware_reset(&self) -> Result<(), Error> {
        Error::call1(rs2::rs2_hardware_reset, self.0)
    }

    /// Check if Advanced-Mode is enabled
    ///
    /// # Returns
    /// * `Ok(bool)` on success.
    /// * `Err(Error)` on failure.
    pub fn is_enabled(&self) -> Result<bool, Error> {
        let mut is_enabled: i32 = -1;
        Error::call2(rs2::rs2_is_enabled, self.0, &mut is_enabled as *mut i32)?;
        Ok(is_enabled == 1)
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
        Error::call2(rs2::rs2_toggle_advanced_mode, self.0, enable as i32)
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
        Error::call3(
            rs2::rs2_load_json,
            self.0,
            json_content.as_ptr() as *const std::os::raw::c_void,
            json_content.len() as u32,
        )
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
        Error::call2(
            rs2::rs2_playback_device_set_real_time,
            self.0,
            enable as i32,
        )
    }

    /// Indicates if playback is in real time mode or non real time.
    ///
    /// connected device.
    /// # Returns
    /// * `Ok(bool)` on success, `true` for real time mode and `false` for non real time mode.
    /// * `Err(Error)` on failure.
    pub fn is_real_time(&self) -> Result<bool, Error> {
        Error::call1(rs2::rs2_playback_device_is_real_time, self.0).map(|r: i32| r != 0)
    }
}
