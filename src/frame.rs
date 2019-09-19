use crate::error::Error;
use crate::stream::StreamProfile;
use rs2;
use crate::metadata::{Metadata, MetadataAttribute};
use std::collections::HashMap;

pub struct Frame {
    pub raw: *mut rs2::rs2_frame,
}

// impl Drop for Frame {
//     fn drop(&mut self) {
//         unsafe { rs2::rs2_release_frame(self.raw); }
//     }
// }

impl Frame {
    pub fn release(&self) {
        unsafe { rs2::rs2_release_frame(self.raw) };
    }

    pub fn get_frame_number(&self) -> Result<u64, Error> {
        let mut error = Error::default();
        let frame_number = unsafe { rs2::rs2_get_frame_number(self.raw, error.inner()) };
        if error.check() {
            Err(error)
        } else {
            Ok(frame_number)
        }
    }

    pub fn get_timestamp(&self) -> Result<f64, Error> {
        let mut error = Error::default();
        let timestamp = unsafe { rs2::rs2_get_frame_timestamp(self.raw, error.inner()) };
        if error.check() {
            Err(error)
        } else {
            Ok(timestamp)
        }
    }

    pub fn get_height(&self) -> Result<i32, Error> {
        let mut error = Error::default();
        let height = unsafe { rs2::rs2_get_frame_height(self.raw, error.inner()) };
        if error.check() {
            Err(error)
        } else {
            Ok(height)
        }
    }

    pub fn get_width(&self) -> Result<i32, Error> {
        let mut error = Error::default();
        let width = unsafe { rs2::rs2_get_frame_width(self.raw, error.inner()) };
        if error.check() {
            Err(error)
        } else {
            Ok(width)
        }
    }

    pub fn get_bits_per_pixel(&self) -> Result<i32, Error> {
        let mut error = Error::default();
        let bpp = unsafe { rs2::rs2_get_frame_bits_per_pixel(self.raw, error.inner()) };
        if error.check() {
            Err(error)
        } else {
            Ok(bpp)
        }
    }

    pub fn get_size(&self) -> Result<usize, Error> {
        let width = self.get_width()?;
        let height = self.get_height()?;
        let bits = self.get_bits_per_pixel()?;
        Ok((width * height * bits) as usize)
    }

    pub fn get_data(&self) -> Result<Vec<u8>, Error> {
        let mut error = Error::default();
        let size = self.get_size().unwrap();
        let data = unsafe {
            let data_ptr = rs2::rs2_get_frame_data(self.raw, error.inner());
            if error.check() {
                return Err(error);
            };
            std::slice::from_raw_parts(data_ptr as *const u8, (size / 8) as usize).to_vec()
        };
        Ok(data)
    }

    pub fn get_profile(&self) -> Result<StreamProfile, Error> {
        let mut error = Error::default();
        let profile = StreamProfile {
            raw: unsafe {
                rs2::rs2_get_frame_stream_profile(self.raw, error.inner())
                    as *mut rs2::rs2_stream_profile
            },
            clone: false,
        };
        if error.check() {
            Err(error)
        } else {
            Ok(profile)
        }
    }

    /// Check if the frame's metadata supports the given attribute. The function returns `true` if
    /// it does, `false` if not. The error variant is used to propagate any errors encountered in
    /// the librealsense code.
    /// # Arguments
    /// * `attribute` - The attribute to check support for.
    /// # Example
    /// ```
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
        let meta_supported = unsafe { rs2::rs2_supports_frame_metadata(self.raw, attribute as rs2::rs2_frame_metadata_value, error.inner()) };
        if error.check() {
            Err(error)
        }
        else if meta_supported == 1 {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    /// Read the given metadata attribute from the frame. Please use the `supports_frame_metadata`
    /// function to check if the given metadata is supported before reading it, as librealsense may
    /// fail with an exception when reading an un-supported metadata attribute.
    /// # Arguments
    /// * `attribute` - The attribute to read.
    /// # Example
    /// ```
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
        let value = unsafe { rs2::rs2_get_frame_metadata(self.raw, attribute as rs2::rs2_frame_metadata_value, error.inner()) };
        if error.check() {
            Err(error)
        }
        else {
            Ok(value)
        }
    }

    /// Get all the frame's supported metadata field represented as a `Metadata` struct.
    /// # Example
    /// ```
    /// use librealsense2::pipeline::Pipeline;
    /// use librealsense2::context::Context;
    /// let pipeline = Pipeline::new(&Context::new().unwrap()).unwrap();
    ///
    /// let frames = pipeline.wait_for_frames(2500).unwrap();
    /// let metadata = frames[0].get_metadata().unwrap();
    /// println!("frames[0]'s contrast is {}", metadata.contrast.unwrap());
    /// ```
    pub fn get_metadata(&self) -> Result<Metadata, Error> {
        let mut error = Error::default();
        let mut meta_values : HashMap<u32, i64> = HashMap::new();

        for i in 0..rs2::rs2_frame_metadata_value_RS2_FRAME_METADATA_COUNT {
            // Cast the integer to a rs2_frame_metadata_value, which realsense uses to identify metadata fields
            let metadata_value : rs2::rs2_frame_metadata_value = i;
            // Check if the given index is supported, ignore it if not
            let meta_supported = unsafe { rs2::rs2_supports_frame_metadata(self.raw, metadata_value, error.inner()) };
            if meta_supported == 0 || error.check() {
                continue;
            }
            // Attempt to get the meta's name and value
            let mete_val = unsafe { rs2::rs2_get_frame_metadata(self.raw, metadata_value, error.inner()) };
            if error.check() {
                return Err(error);
            }

            // Append that to the dictionary
            meta_values.insert(metadata_value, mete_val);
        }
        Ok(Metadata::from(meta_values))
    }
}
