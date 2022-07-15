// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.
use crate::device::{Device, DeviceList};
use crate::error::Error;

/// Struct representation of [`Context`](../context/struct.Context.html) that wraps
/// around `rs2_context` handle. The [`Context`](../context/struct.Context.html) is
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

unsafe impl Send for Context {}

impl Context {
    /// Creates `RealSense` [`Context`](../context/struct.Context.html) that is
    /// required for the rest of the API, while utlising the current version.
    ///
    /// # Returns
    /// * `Ok(Context)` on success.
    /// * `Err(Error)` on failure.
    pub fn new() -> Result<Context, Error> {
        let mut error = Error::default();
        let context = Context {
            handle: unsafe { rs2::rs2_create_context(rs2::RS2_API_VERSION as i32, error.inner()) },
        };
        error.check()?;
        Ok(context)
    }

    /// Creates `RealSense` [`Context`](../context/struct.Context.html) that is
    /// required for the rest of the API, while utlising the current version.
    ///
    /// # Returns
    /// * `Ok(Context)` on success.
    /// * `Err(Error)` on failure.
    pub fn query_devices(&self) -> Result<Vec<Device>, Error> {
        let mut error = Error::default();
        let device_list = DeviceList {
            handle: unsafe { rs2::rs2_query_devices(self.handle, error.inner()) },
        };
        error.check()?;

        let mut error = Error::default();
        let count = unsafe { rs2::rs2_get_device_count(device_list.handle, error.inner()) };
        error.check()?;

        let mut res: Vec<Device> = Vec::new();
        for i in 0..count {
            let mut error = Error::default();
            res.push(Device {
                handle: unsafe { rs2::rs2_create_device(device_list.handle, i, error.inner()) },
            });
            error.check()?;
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
}
