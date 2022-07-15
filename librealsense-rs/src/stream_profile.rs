// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.

use std::fmt::{Display, Formatter};

use crate::error::Error;
use crate::extrinsics::*;
use crate::intrinsics::*;

/// Struct representation of [`StreamProfile`](../stream_profile/struct.Pipeline.html) that wraps
/// around `rs2_stream_profile` handle. The
/// [`StreamProfile`](../stream_profile/struct.Pipeline.html) contains information about a specific
/// stream.
pub struct StreamProfile {
    pub(crate) handle: *mut rs2::rs2_stream_profile,
}

pub struct StreamProfileList {
    pub(crate) handle: *mut rs2::rs2_stream_profile_list,
}

impl Drop for StreamProfileList {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_delete_stream_profiles_list(self.handle);
        }
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
#[derive(PartialEq, Clone, Debug)]
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
        let mut error = Error::default();
        let mut data = StreamData::default();
        unsafe {
            rs2::rs2_get_stream_profile_data(
                self.handle,
                &mut data.stream as *mut rs2::rs2_stream,
                &mut data.format as *mut rs2::rs2_format,
                &mut data.index as *mut i32,
                &mut data.id as *mut i32,
                &mut data.framerate as *mut i32,
                error.inner(),
            );
        }
        error.check()?;
        Ok(data)
    }

    /// Extract resolution of the stream described by
    /// [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    ///
    /// # Returns
    /// * `Ok(StreamResolution)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_resolution(&self) -> Result<StreamResolution, Error> {
        let mut error = Error::default();
        let mut resolution = StreamResolution::default();
        unsafe {
            rs2::rs2_get_video_stream_resolution(
                self.handle,
                &mut resolution.width as *mut i32,
                &mut resolution.height as *mut i32,
                error.inner(),
            );
        }
        error.check()?;
        Ok(resolution)
    }

    /// Obtain intrinsics of a [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    ///
    /// # Returns
    /// * `Ok(Intrinsics)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_intrinsics(&self) -> Result<Intrinsics, Error> {
        let mut error = Error::default();
        let mut intrinsics = RsIntrinsicsWrapper::default();
        unsafe {
            rs2::rs2_get_video_stream_intrinsics(
                self.handle,
                &mut intrinsics._handle,
                error.inner(),
            );
        }
        error.check()?;
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
        let mut error = Error::default();
        let mut extrinsics = RsExtrinsicsWrapper::default();
        unsafe {
            rs2::rs2_get_extrinsics(
                from.handle,
                to.handle,
                &mut extrinsics._handle,
                error.inner(),
            );
        }
        error.check()?;
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
