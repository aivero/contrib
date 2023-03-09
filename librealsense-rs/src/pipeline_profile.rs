use crate::device::Device;
use crate::error::Error;
use crate::stream_profile::*;

/// Struct representation of [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html)
/// that wraps around `rs2_pipeline_profile` handle.
pub struct PipelineProfile(pub(crate) *mut rs2::rs2_pipeline_profile);

/// Safe releasing of the `rs2_pipeline_profile` handle.
impl Drop for PipelineProfile {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_pipeline_profile(self.0) }
    }
}

impl From<*mut rs2::rs2_pipeline_profile> for PipelineProfile {
    fn from(p: *mut rs2::rs2_pipeline_profile) -> Self {
        PipelineProfile(p)
    }
}

impl PipelineProfile {
    /// Retrieve the [`Device`](../device/struct.Device.html) used by the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html). The [`Device`](../device/struct.Device.html)
    /// class provides the application access to control camera additional settings - get
    /// [`Device`](../device/struct.Device.html) information, sensor options information, options
    /// value query and set, sensor specific extensions. Since the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) controls the
    /// [`Device`](../device/struct.Device.html) streams configuration, activation state and frames
    /// reading, calling the [`Device`](../device/struct.Device.html) API functions, which execute
    /// those operations, results in unexpected behavior. The
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) streaming
    /// [`Device`](../device/struct.Device.html) is selected during
    /// [`Pipeline::start()`](../pipeline/struct.Pipeline.html#method.start).
    /// [`Device`](../device/struct.Device.html)s of profiles, which are not returned by
    /// [`Pipeline::start()`](../pipeline/struct.Pipeline.html#method.start) or
    /// [`Pipeline::get_active_profile()`](../pipeline/struct.Pipeline.html#method.get_active_profile)
    /// , are not guaranteed to be used by the [`Pipeline`](../pipeline/struct.Pipeline.html).
    ///
    /// # Returns
    /// * `Ok(Device)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_device(&self) -> Result<Device, Error> {
        Error::call1(rs2::rs2_pipeline_profile_get_device, self.0)
    }

    /// Retrieve the selected [`StreamProfile`](../stream_profile/struct.StreamProfile.html)s,
    /// which are enabled in this
    /// [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html).
    ///
    /// # Returns
    /// * `Ok(StreamProfileList)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_streams(&self) -> Result<StreamProfileList, Error> {
        Error::call1(rs2::rs2_pipeline_profile_get_streams, self.0)
    }
}
