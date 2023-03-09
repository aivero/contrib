use crate::error::Error;
use crate::metadata::{Metadata, MetadataAttribute};
use crate::stream_profile::StreamProfile;
use std::collections::HashMap;

/// Struct representation of [`Frame`](../frame/struct.Frame.html) that wraps around
/// `rs2_frame` handle.
pub struct Frame(pub(crate) *mut rs2::rs2_frame);

/// Safe releasing of the `rs2_frame` handle.
impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                rs2::rs2_release_frame(self.0);
            }
        }
    }
}

impl From<*mut rs2::rs2_frame> for Frame {
    fn from(f: *mut rs2::rs2_frame) -> Self {
        Frame(f)
    }
}

impl Frame {
    /// Extract individual frames from a frameset.
    ///
    /// # Returns
    /// * `Ok(Frame)` on success.
    /// * `Err(Error)` on failure.
    pub fn extract_frame(&self, i: i32) -> Result<Frame, Error> {
        Error::call2(rs2::rs2_extract_frame, self.0, i)
    }

    /// Get number of frames embedded within a composite frame
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn embedded_frames_count(&self) -> Result<i32, Error> {
        Error::call1(rs2::rs2_embedded_frames_count, self.0)
    }

    /// Retrieve timestamp from [`Frame`](../frame/struct.Frame.html) in milliseconds.
    ///
    /// # Returns
    /// * `Ok(f64)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_timestamp(&self) -> Result<f64, Error> {
        Error::call1(rs2::rs2_get_frame_timestamp, self.0)
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
        Error::call1(rs2::rs2_get_frame_timestamp_domain, self.0)
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
    /// let pipeline = Pipeline::create(&Context::create().unwrap()).unwrap();
    ///
    /// let frames = pipeline.wait_for_frames(2500).unwrap();
    /// let frame = frames.extract_frame(0).unwrap();
    /// let contrast = if frame.supports_frame_metadata(MetadataAttribute::Contrast).unwrap() {
    ///     Some(frame.get_frame_metadata(MetadataAttribute::Contrast).unwrap())
    /// } else {
    ///     None
    /// };
    /// ```
    pub fn get_frame_metadata(&self, attribute: MetadataAttribute) -> Result<i64, Error> {
        Error::call2(
            rs2::rs2_get_frame_metadata,
            self.0,
            attribute as rs2::rs2_frame_metadata_value,
        )
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
    /// let pipeline = Pipeline::create(&Context::create().unwrap()).unwrap();
    ///
    /// let frames = pipeline.wait_for_frames(2500).unwrap();
    /// if frames.extract_frame(0).unwrap().supports_frame_metadata(MetadataAttribute::Contrast).unwrap() {
    ///     println!("frames[0] supports the 'Contrast' metadata.")
    /// }
    /// else { println!("frames[0] does not support the 'Contrast' metadata.") };
    /// ```
    pub fn supports_frame_metadata(&self, attribute: MetadataAttribute) -> Result<bool, Error> {
        Error::call2(
            rs2::rs2_supports_frame_metadata,
            self.0,
            attribute as rs2::rs2_frame_metadata_value,
        )
        .map(|m: i32| m == 1)
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
    /// let pipeline = Pipeline::create(&Context::create().unwrap()).unwrap();
    ///
    /// let frames = pipeline.wait_for_frames(2500).unwrap();
    /// let metadata = frames.extract_frame(0).unwrap().get_metadata().unwrap();
    /// println!("frames[0]'s contrast is {}", metadata.contrast.unwrap());
    /// ```
    pub fn get_metadata(&self) -> Result<Metadata, Error> {
        let mut meta_values: HashMap<u32, i64> = HashMap::new();

        for i in 0..rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_COUNT {
            // Cast the integer to a rs2_frame_metadata_value, which realsense uses to identify metadata fields
            let metadata_value: rs2::rs2_frame_metadata_value = i;
            // Check if the given index is supported, ignore it if not
            let meta_supported: Result<i32, Error> =
                Error::call2(rs2::rs2_supports_frame_metadata, self.0, metadata_value);
            if meta_supported.map(|m| m == 0).unwrap_or(true) {
                continue;
            }
            // Attempt to get the meta's name and value
            let mete_val = Error::call2(rs2::rs2_get_frame_metadata, self.0, metadata_value)?;
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
        Error::call1(rs2::rs2_get_frame_number, self.0)
    }

    /// Retrieve the height of a [`Frame`](../frame/struct.Frame.html) in pixels.
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_height(&self) -> Result<i32, Error> {
        Error::call1(rs2::rs2_get_frame_height, self.0)
    }

    /// Retrieve the width of a [`Frame`](../frame/struct.Frame.html) in pixels.
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_width(&self) -> Result<i32, Error> {
        Error::call1(rs2::rs2_get_frame_width, self.0)
    }

    /// Retrieve bits per pixels in the [`Frame`](../frame/struct.Frame.html) image
    /// (note that bits per pixel is not necessarily divided by 8, as in 12bpp)
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_bits_per_pixel(&self) -> Result<i32, Error> {
        Error::call1(rs2::rs2_get_frame_bits_per_pixel, self.0)
    }

    /// Retrieve [`Frame`](../frame/struct.Frame.html) stride in bytes (number of bytes
    /// from start of line to start of next line).
    ///
    /// # Returns
    /// * `Ok(i32)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_stride(&self) -> Result<i32, Error> {
        Error::call1(rs2::rs2_get_frame_stride_in_bytes, self.0)
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
        Error::call1(rs2::rs2_get_frame_data_size, self.0)
    }

    /// Retrieve the data from [`Frame`](../frame/struct.Frame.html).
    ///
    /// # Returns
    /// * `Ok(&[u8])` on success.
    /// * `Err(Error)` on failure.
    pub fn get_data(&self) -> Result<&[u8], Error> {
        let data = {
            let data_ptr: *const std::ffi::c_void = Error::call1(rs2::rs2_get_frame_data, self.0)?;
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
        Error::call1(rs2::rs2_get_frame_stream_profile, self.0)
    }
}
