// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.

use std::fmt::{Display, Formatter};

use crate::error::Error;
use crate::extrinsics::*;
use crate::intrinsics::*;

pub struct StreamProfileList(pub(crate) *mut rs2::rs2_stream_profile_list);

impl Drop for StreamProfileList {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_stream_profiles_list(self.0) }
    }
}

impl From<*mut rs2::rs2_stream_profile_list> for StreamProfileList {
    fn from(l: *mut rs2::rs2_stream_profile_list) -> Self {
        StreamProfileList(l)
    }
}

impl StreamProfileList {
    /// Get the number of supported stream profiles
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn count(&self) -> Result<i32, Error> {
        Error::call1(rs2::rs2_get_stream_profiles_count, self.0)
    }

    /// Get pointer to specific stream profile
    ///
    /// # Arguments
    /// * `index` - the zero based index of the streaming mode
    ///
    /// # Returns
    /// * `Ok(StreamProfile)` on success.
    /// * `Err(Error)` on failure.
    pub fn get(&self, i: i32) -> Result<StreamProfile, Error> {
        Error::call2(rs2::rs2_get_stream_profile, self.0, i)
    }
}

/// Struct representation of [`StreamProfile`](../stream_profile/struct.Pipeline.html) that wraps
/// around `rs2_stream_profile` handle. The
/// [`StreamProfile`](../stream_profile/struct.Pipeline.html) contains information about a specific
/// stream.
pub struct StreamProfile(pub(crate) *const rs2::rs2_stream_profile);

impl From<*const rs2::rs2_stream_profile> for StreamProfile {
    fn from(p: *const rs2::rs2_stream_profile) -> Self {
        StreamProfile(p)
    }
}

/// Helper struct that contains data from [`StreamProfile`](../stream_profile/struct.Pipeline.html).
#[derive(Debug)]
pub struct StreamData {
    pub stream: rs2::rs2_stream,
    pub format: rs2::rs2_format,
    pub index: i32,
    pub id: i32,
    pub framerate: i32,
}

/// Default constructor of `StreamData`.
impl Default for StreamData {
    fn default() -> Self {
        Self {
            stream: rs2::rs2_stream::RS2_STREAM_ANY,
            format: rs2::rs2_format::RS2_FORMAT_ANY,
            index: -1,
            id: 0,
            framerate: 0,
        }
    }
}

/// Helper struct that contains resolution from
/// [`StreamResolution`](../stream_profile/struct.StreamResolution.html).
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct StreamResolution {
    pub width: i32,
    pub height: i32,
}

/// Default constructor of [`StreamResolution`](../stream_profile/struct.StreamResolution.html).
impl Default for StreamResolution {
    fn default() -> Self {
        Self {
            width: -1,
            height: -1,
        }
    }
}

impl Display for StreamResolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}px", self.width, self.height)
    }
}

impl StreamResolution {
    /// Constructor of [`StreamResolution`](../stream_profile/struct.StreamResolution.html) with specified `width` and `height`.
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

impl StreamProfile {
    /// Extract common parameters of a [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    ///
    /// # Returns
    /// * `Ok(StreamData)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_data(&self) -> Result<StreamData, Error> {
        let mut data = StreamData::default();
        Error::call6(
            rs2::rs2_get_stream_profile_data,
            self.0,
            &mut data.stream as *mut rs2::rs2_stream,
            &mut data.format as *mut rs2::rs2_format,
            &mut data.index as *mut i32,
            &mut data.id as *mut i32,
            &mut data.framerate as *mut i32,
        )?;
        Ok(data)
    }

    /// Extract resolution of the stream described by
    /// [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    ///
    /// # Returns
    /// * `Ok(StreamResolution)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_resolution(&self) -> Result<StreamResolution, Error> {
        let mut resolution = StreamResolution::default();
        Error::call3(
            rs2::rs2_get_video_stream_resolution,
            self.0,
            &mut resolution.width as *mut i32,
            &mut resolution.height as *mut i32,
        )?;
        Ok(resolution)
    }

    /// Obtain intrinsics of a [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    ///
    /// # Returns
    /// * `Ok(Intrinsics)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_intrinsics(&self) -> Result<Intrinsics, Error> {
        let mut intrinsics = RsIntrinsicsWrapper::default();
        Error::call2(
            rs2::rs2_get_video_stream_intrinsics,
            self.0,
            &mut intrinsics._handle,
        )?;
        Ok(Intrinsics::new(intrinsics._handle))
    }

    /// Obtain extrinsics between two [`StreamProfile`](../stream_profile/struct.Pipeline.html)s.
    ///
    /// # Arguments
    /// * `from` - Origin [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    /// * `to` - Target [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    ///
    /// # Returns
    /// * `Ok(Extrinsics)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_extrinsics(from: &Self, to: &Self) -> Result<Extrinsics, Error> {
        let mut extrinsics = RsExtrinsicsWrapper::default();
        Error::call3(
            rs2::rs2_get_extrinsics,
            from.0,
            to.0,
            &mut extrinsics._handle,
        )?;
        Ok(Extrinsics::new(extrinsics._handle))
    }

    /// Obtain extrinsics to another [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    ///
    /// # Arguments
    /// * `target` - Target [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    ///
    /// # Returns
    /// * `Ok(Extrinsics)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_extrinsics_to(&self, target: &Self) -> Result<Extrinsics, Error> {
        Self::get_extrinsics(self, target)
    }
}
