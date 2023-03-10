use crate::device::DeviceList;
use crate::error::Error;

/// Struct representation of [`Context`](../context/struct.Context.html) that wraps
/// around `rs2_context` handle. The [`Context`](../context/struct.Context.html) is
/// required for the rest of the API.
pub struct Context(pub(crate) *mut rs2::rs2_context);

/// Safe releasing of the `rs2_context` handle.
impl Drop for Context {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_context(self.0) }
    }
}

impl From<*mut rs2::rs2_context> for Context {
    fn from(c: *mut rs2::rs2_context) -> Self {
        Context(c)
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
    pub fn create() -> Result<Context, Error> {
        Error::call1(rs2::rs2_create_context, rs2::RS2_API_VERSION as i32)
    }

    /// Creates `RealSense` [`Context`](../context/struct.Context.html) that is
    /// required for the rest of the API, while utlising the current version.
    ///
    /// # Returns
    /// * `Ok(Context)` on success.
    /// * `Err(Error)` on failure.
    pub fn query_devices(&self) -> Result<DeviceList, Error> {
        Error::call1(rs2::rs2_query_devices, self.0)
    }
}
