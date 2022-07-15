// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.
use crate::error::Error;
use crate::metadata::{Metadata, MetadataAttribute};
use crate::sensor::Sensor;
use crate::stream_profile::StreamProfile;
use std::collections::HashMap;

/// Struct representation of [`Frame`](../frame/struct.Frame.html) that wraps around
/// `rs2_frame` handle.
pub struct Frame {
    pub(crate) handle: *mut rs2::rs2_frame,
}

/// Safe releasing of the `rs2_frame` handle.
impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            if !self.handle.is_null() {
                rs2::rs2_release_frame(self.handle);
            }
        }
    }
}

impl Frame {
    #[deprecated(since = "0.6.0", note = "Proper Rust dropping is utilised")]
    pub fn release(&self) {
        // unsafe { rs2::rs2_release_frame(self.handle) };
    }

    /// Extract individual frames from a frameset.
    ///
    /// # Returns
    /// * `Ok(Vec<Frame>)` on success.
    /// * `Err(Error)` on failure.
    pub fn extract_frames(&self) -> Result<Vec<Frame>, Error> {
        let mut error = Error::default();
        let count = unsafe { rs2::rs2_embedded_frames_count(self.handle, error.inner()) };
        error.check()?;

        let mut res: Vec<Frame> = Vec::new();
        for i in 0..count {
            let mut error = Error::default();
            res.push(Frame {
                handle: unsafe { rs2::rs2_extract_frame(self.handle, i, error.inner()) },
            });
            error.check()?;
        }
        Ok(res)
    }

    /// Retrieve [`Frame`](../frame/struct.Frame.html)'s parent
    /// [`Sensor`](../sensor/struct.Sensor.html).
    ///
    /// # Returns
    /// * `Ok(Sensor)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_sensor(&self) -> Result<Sensor, Error> {
        unimplemented!();
    }

    /// Retrieve timestamp from [`Frame`](../frame/struct.Frame.html) in milliseconds.
    ///
    /// # Returns
    /// * `Ok(f64)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_timestamp(&self) -> Result<f64, Error> {
        let mut error = Error::default();
        let timestamp = unsafe { rs2::rs2_get_frame_timestamp(self.handle, error.inner()) };
        error.check()?;
        Ok(timestamp)
    }

    /// Retrieve timestamp domain from [`Frame`](../frame/struct.Frame.html).
    /// Timestamps can only be comparable if they are in common domain (for example, depth
    /// timestamp might come from system time while color timestamp might come from the device)
    /// this method is used to check if two timestamp values are comparable (generated from the
    /// same clock).
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn get_timestamp_domain(&self) -> Result<rs2::rs2_timestamp_domain, Error> {
        let mut error = Error::default();
        let timestamp_domain =
            unsafe { rs2::rs2_get_frame_timestamp_domain(self.handle, error.inner()) };
        error.check()?;
        Ok(timestamp_domain)
    }

    /// Read the given metadata attribute from the
    /// [`Frame`](../frame/struct.Frame.html). Please use the
    /// [`Frame::supports_frame_metadata()`](../frame/struct.Frame.html#method.supports_frame_metadata)
    /// function to check if the given metadata is supported before reading it, as librealsense may
    /// fail with an exception when reading an un-supported metadata attribute. Please refer to
    /// [`Frame::get_metadata()`](../frame/struct.Frame.html#method.get_metadata) for a
    /// Rustified version.
    ///
    /// # Arguments
    /// * `attribute` - The attribute to read.
    ///
    /// # Returns
    /// * `Ok(i64)` on success.
    /// * `Err(Error)` on failure.
    ///
    /// # Example
    /// ```no_run
    /// use librealsense2::pipeline::Pipeline;
    /// use librealsense2::context::Context;
    /// use librealsense2::metadata::MetadataAttribute;
    /// let pipeline = Pipeline::new(&Context::new().unwrap()).unwrap();
    ///
    /// let frames = pipeline.wait_for_frames(2500).unwrap();
    /// let contrast =
    ///     if frames[0].supports_frame_metadata(MetadataAttribute::Contrast).unwrap() {
    ///         Some(frames[0].get_frame_metadata(MetadataAttribute::Contrast).unwrap())
    ///     }
    ///     else { None };
    /// ```
    pub fn get_frame_metadata(&self, attribute: MetadataAttribute) -> Result<i64, Error> {
        let mut error = Error::default();
        let value = unsafe {
            rs2::rs2_get_frame_metadata(
                self.handle,
                attribute as rs2::rs2_frame_metadata_value,
                error.inner(),
            )
        };
        error.check()?;
        Ok(value)
    }

    /// Check if the [`Frame`](../frame/struct.Frame.html)'s metadata supports the
    /// given attribute. Please refer to
    /// [`Frame::get_metadata()`](../frame/struct.Frame.html#method.get_metadata)
    /// for a Rustified version.
    ///
    /// # Arguments
    /// * `attribute` - The attribute to check support for.
    ///
    /// # Returns
    /// * `Ok(bool)` on success, `true` if supported and `false` if not.
    /// * `Err(Error)` on failure.
    ///
    /// # Example
    /// ```no_run
    /// use librealsense2::pipeline::Pipeline;
    /// use librealsense2::context::Context;
    /// use librealsense2::metadata::MetadataAttribute;
    /// let pipeline = Pipeline::new(&Context::new().unwrap()).unwrap();
    ///
    /// let frames = pipeline.wait_for_frames(2500).unwrap();
    /// if frames[0].supports_frame_metadata(MetadataAttribute::Contrast).unwrap() {
    ///     println!("frames[0] supports the 'Contrast' metadata.")
    /// }
    /// else { println!("frames[0] does not support the 'Contrast' metadata.") };
    /// ```
    pub fn supports_frame_metadata(&self, attribute: MetadataAttribute) -> Result<bool, Error> {
        let mut error = Error::default();
        let meta_supported = unsafe {
            rs2::rs2_supports_frame_metadata(
                self.handle,
                attribute as rs2::rs2_frame_metadata_value,
                error.inner(),
            )
        };
        error.check()?;
        Ok(meta_supported == 1)
    }

    /// Get all the frame's supported metadata field represented as a `Metadata` struct. Please
    /// refer to
    /// [`Frame::supports_frame_metadata()`](../frame/struct.Frame.html#method.supports_frame_metadata)
    /// or [`Frame::get_frame_metadata()`](../frame/struct.Frame.html#method.get_frame_metadata)
    /// for the C-like variants.
    ///
    /// # Returns
    /// * `Ok(Metadata)` on success.
    /// * `Err(Error)` on failure.
    ///
    /// # Example
    /// ```no_run
    /// use librealsense2::pipeline::Pipeline;
    /// use librealsense2::context::Context;
    /// let pipeline = Pipeline::new(&Context::new().unwrap()).unwrap();
    ///
    /// let frames = pipeline.wait_for_frames(2500).unwrap();
    /// let metadata = frames[0].get_metadata().unwrap();
    /// println!("frames[0]'s contrast is {}", metadata.contrast.unwrap());
    /// ```
    pub fn get_metadata(&self) -> Result<Metadata, Error> {
        let mut meta_values: HashMap<u32, i64> = HashMap::new();

        for i in 0..rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_COUNT {
            // Cast the integer to a rs2_frame_metadata_value, which realsense uses to identify metadata fields
            let metadata_value: rs2::rs2_frame_metadata_value = i;
            // Check if the given index is supported, ignore it if not
            let mut error = Error::default();
            let meta_supported = unsafe {
                rs2::rs2_supports_frame_metadata(self.handle, metadata_value, error.inner())
            };
            if meta_supported == 0 || error.check().is_err() {
                continue;
            }
            // Attempt to get the meta's name and value
            let mut error = Error::default();
            let mete_val =
                unsafe { rs2::rs2_get_frame_metadata(self.handle, metadata_value, error.inner()) };
            error.check()?;

            // Append that to the dictionary
            meta_values.insert(metadata_value, mete_val);
        }
        Ok(Metadata::from(meta_values))
    }

    /// Retrieve the [`Frame`](../frame/struct.Frame.html) number.
    ///
    /// # Returns
    /// * `Ok(u64)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_frame_number(&self) -> Result<u64, Error> {
        let mut error = Error::default();
        let frame_number = unsafe { rs2::rs2_get_frame_number(self.handle, error.inner()) };
        error.check()?;
        Ok(frame_number)
    }

    /// Retrieve the height of a [`Frame`](../frame/struct.Frame.html) in pixels.
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_height(&self) -> Result<i32, Error> {
        let mut error = Error::default();
        let height = unsafe { rs2::rs2_get_frame_height(self.handle, error.inner()) };
        error.check()?;
        Ok(height)
    }

    /// Retrieve the width of a [`Frame`](../frame/struct.Frame.html) in pixels.
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_width(&self) -> Result<i32, Error> {
        let mut error = Error::default();
        let width = unsafe { rs2::rs2_get_frame_width(self.handle, error.inner()) };
        error.check()?;
        Ok(width)
    }

    /// Retrieve bits per pixels in the [`Frame`](../frame/struct.Frame.html) image
    /// (note that bits per pixel is not necessarily divided by 8, as in 12bpp)
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_bits_per_pixel(&self) -> Result<i32, Error> {
        let mut error = Error::default();
        let bpp = unsafe { rs2::rs2_get_frame_bits_per_pixel(self.handle, error.inner()) };
        error.check()?;
        Ok(bpp)
    }

    /// Retrieve [`Frame`](../frame/struct.Frame.html) stride in bytes (number of bytes
    /// from start of line to start of next line).
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_stride(&self) -> Result<i32, Error> {
        let mut error = Error::default();
        let stride = unsafe { rs2::rs2_get_frame_stride_in_bytes(self.handle, error.inner()) };
        error.check()?;
        Ok(stride)
    }

    /// Retrieve the data size of a [`Frame`](../frame/struct.Frame.html) in bytes.
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_size(&self) -> Result<i32, Error> {
        let width = self.get_width()?;
        let height = self.get_height()?;
        let bits = self.get_bits_per_pixel()?;
        Ok(width * height * bits)
    }

    /// Retrieve the size of a [`Frame`](../frame/struct.Frame.html) in memory.
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_data_size(&self) -> Result<i32, Error> {
        let mut error = Error::default();
        let size = unsafe { rs2::rs2_get_frame_data_size(self.handle, error.inner()) };
        error.check()?;
        Ok(size)
    }

    /// Retrieve the data from [`Frame`](../frame/struct.Frame.html).
    ///
    /// # Returns
    /// * `Ok(&[u8])` on success.
    /// * `Err(Error)` on failure.
    pub fn get_data(&self) -> Result<&[u8], Error> {
        let mut error = Error::default();
        let data = {
            let data_ptr = unsafe { rs2::rs2_get_frame_data(self.handle, error.inner()) };
            error.check()?;
            let size = self.get_data_size()? as usize;
            unsafe { std::slice::from_raw_parts(data_ptr as *const u8, size) }
        };
        Ok(data)
    }

    /// Retrieve the [`StreamProfile`](../stream_profile/struct.StreamProfile.html) that
    /// was used to start the stream of this [`Frame`](../frame/struct.Frame.html).
    ///
    /// # Returns
    /// * `Ok(StreamProfile)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_stream_profile(&self) -> Result<StreamProfile, Error> {
        let mut error = Error::default();
        let profile = StreamProfile {
            handle: unsafe {
                rs2::rs2_get_frame_stream_profile(self.handle, error.inner())
                    as *mut rs2::rs2_stream_profile
            },
        };
        error.check()?;
        Ok(profile)
    }
}
