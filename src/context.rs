use crate::device::Device;
use crate::error::Error;
use crate::record_playback::Playback;
use crate::sensor::Sensor;
use rs2;

/// Struct representation of `Context` that wraps around `rs2_context` handle. The `Context` is
/// required for the rest of the API.
pub struct Context {
    pub(crate) handle: *mut rs2::rs2_context,
}

/// Safe releasing of the `rs2_context` handle.
impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_delete_context(self.handle);
        }
    }
}

impl Context {
    /// Creates `RealSense` `Context` that is required for the rest of the API, while utlising the
    /// current version.
    ///
    /// # Returns
    /// * `Ok(Context)` on success.
    /// * `Err(Error)` on failure.
    pub fn new() -> Result<Context, Error> {
        let mut error = Error::default();
        let context = Context {
            handle: unsafe { rs2::rs2_create_context(rs2::RS2_API_VERSION as i32, error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(context)
        }
    }

    /// Creates `RealSense` `Context` that is required for the rest of the API, while utlising the
    /// current version.
    ///
    /// # Returns
    /// * `Ok(Context)` on success.
    /// * `Err(Error)` on failure.
    pub fn query_devices(&self) -> Result<Vec<Device>, Error> {
        let mut error = Error::default();
        let device_list = unsafe { rs2::rs2_query_devices(self.handle, error.inner()) };
        if error.check() {
            return Err(error);
        };

        let count = unsafe { rs2::rs2_get_device_count(device_list, error.inner()) };
        let mut res: Vec<Device> = Vec::new();
        for i in 0..count {
            res.push(Device {
                handle: unsafe { rs2::rs2_create_device(device_list, i, error.inner()) },
            });
            if error.check() {
                return Err(error);
            };
        }
        Ok(res)
    }

    #[deprecated(
        since = "0.6.0",
        note = "Use `query_devices()` to be consistent with C/C++ API"
    )]
    pub fn get_devices(&self) -> Result<Vec<Device>, Error> {
        self.query_devices()
    }

    pub fn query_all_sensors(&self) -> Result<Vec<Sensor>, Error> {
        unimplemented!()
    }

    pub fn get_sensor_parent(&self, _sensor: &Sensor) -> Result<Device, Error> {
        unimplemented!()
    }

    pub fn set_devices_changed_callback(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn load_device(&self, _file: &str) -> Result<Playback, Error> {
        unimplemented!()
    }

    pub fn unload_device(&self, _file: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn unload_tracking_module(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
