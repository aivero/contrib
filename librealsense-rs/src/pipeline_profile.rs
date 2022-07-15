// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.

use crate::device::Device;
use crate::error::Error;
use crate::stream_profile::*;

/// Struct representation of [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html)
/// that wraps around `rs2_pipeline_profile` handle.
pub struct PipelineProfile {
    pub(crate) handle: *mut rs2::rs2_pipeline_profile,
}

/// Safe releasing of the `rs2_pipeline_profile` handle.
impl Drop for PipelineProfile {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_delete_pipeline_profile(self.handle);
        }
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
        let mut error = Error::default();
        let device = Device {
            handle: unsafe { rs2::rs2_pipeline_profile_get_device(self.handle, error.inner()) },
        };
        error.check()?;
        Ok(device)
    }

    /// Retrieve the selected [`StreamProfile`](../stream_profile/struct.StreamProfile.html)s,
    /// which are enabled in this
    /// [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html).
    ///
    /// # Returns
    /// * `Ok(Vec<StreamProfile>)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_streams(&self) -> Result<Vec<StreamProfile>, Error> {
        let mut error = Error::default();
        let stream_profiles = StreamProfileList {
            handle: unsafe { rs2::rs2_pipeline_profile_get_streams(self.handle, error.inner()) },
        };
        error.check()?;

        let mut error = Error::default();
        let stream_count =
            unsafe { rs2::rs2_get_stream_profiles_count(stream_profiles.handle, error.inner()) };
        error.check()?;

        let mut streams: Vec<StreamProfile> = Vec::new();
        streams.reserve_exact(stream_count as usize);

        for i in 0..stream_count {
            let mut error = Error::default();
            streams.push(StreamProfile {
                handle: unsafe {
                    rs2::rs2_get_stream_profile(stream_profiles.handle, i, error.inner())
                        as *mut rs2::rs2_stream_profile
                },
            });
            error.check()?;
        }
        Ok(streams)
    }

    /// Retrieve the [`StreamProfile`](../stream_profile/struct.StreamProfile.html) that is enabled
    /// for the specified stream in this
    /// [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html).
    ///
    /// # Arguments
    /// * `stream_type` - Stream type of the desired profile.
    /// * `stream_index` - Stream index of the desired profile, with -1 for any matching.
    ///
    /// # Returns
    /// * `Ok(StreamProfile)` on success if such stream exist.
    /// * `Ok(None)` on success if no such stream exists.
    /// * `Err(Error)` on failure.
    pub fn get_stream(
        &self,
        stream_type: rs2::rs2_stream,
        stream_index: i32,
    ) -> Result<Option<StreamProfile>, Error> {
        for stream in self.get_streams()? {
            let stream_data = stream.get_data()?;
            if stream_data.stream == stream_type
                && stream_index != -1
                && stream_data.index == stream_index
            {
                return Ok(Some(stream));
            }
        }
        Ok(None)
    }
}
