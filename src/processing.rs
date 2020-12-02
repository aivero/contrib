// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.

use crate::error::Error;
use crate::frame::Frame;
use rs2::rs2_stream;

/// Struct representation of [`ProcessingBlock`](../processing/struct.ProcessingBlock.html) that wraps around
/// `rs2_processing_block` handle.
pub struct ProcessingBlock {
    pub(crate) handle: *mut rs2::rs2_processing_block,
}

/// Safe releasing of the `rs2_processing_block` handle.
impl Drop for ProcessingBlock {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_delete_processing_block(self.handle);
        }
    }
}

impl ProcessingBlock {
    /// This method is used to pass frame into a processing block.
    ///
    /// # Arguments
    /// * `frame` - Frame to process
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn process_frame(&self, frame: &mut Frame) -> Result<(), Error> {
        let mut error = Error::default();
        unsafe {
            rs2::rs2_process_frame(self.handle, frame.handle, error.inner());
            if error.check() {
                return Err(error);
            };
            Ok(())
        }
    }

    /// Creates Align processing block.
    ///
    /// # Arguments
    /// * `align_to` - stream type to be used as the target of frameset alignment
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_align(align_to: rs2_stream) -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_align(align_to, error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates Depth-Colorizer processing block that can be used to quickly visualize the depth data.
    /// This block will accept depth frames as input and replace them by depth frames with format RGB8
    /// Non-depth frames are passed through Further customization will be added soon (format, color-map,
    /// histogram equalization control).
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_colorizer() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_colorizer(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates Depth post-processing filter block. This block accepts depth frames, applies decimation
    /// filter and plots modified prames Note that due to the modifiedframe size, the decimated frame
    /// repaces the original one.
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_decimation_filter() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_decimation_filter_block(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates a post processing block that provides for depth<->disparity domain transformation
    /// for stereo-based depth modules
    ///
    /// # Arguments
    /// * `transform_to_disparity` - flag select the transform direction: true = depth->disparity, and vice versa
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_disparity_transform(transform_to_disparity: bool) -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe {
                rs2::rs2_create_disparity_transform_block(
                    transform_to_disparity as u8,
                    error.inner(),
                )
            },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates Depth post-processing hole filling block. The filter replaces empty pixels with
    /// data from adjacent pixels based on the method selected
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_hole_filling_filter() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_hole_filling_filter_block(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates Depth frame decompression module. Decoded frames compressed and transmitted with Z16H
    /// variable-lenght Huffman code to standartized Z16 Depth data format. Using the compression allows
    /// to reduce the Depth frames bandwidth by more than 50 percent
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_huffman_depth_decompress() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_huffman_depth_decompress_block(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates Point-Cloud processing block. This block accepts depth frames and outputs Points frames.
    /// In addition, given non-depth frame, the block will align texture coordinate to the non-depth stream
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_pointcloud() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_pointcloud(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates a rates printer block. The printer prints the actual FPS of the invoked frame stream.
    /// The block ignores reapiting frames and calculats the FPS only if the frame number of the relevant
    /// frame was changed.
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_rates_printer() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_rates_printer_block(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates Depth post-processing spatial filter block. This block accepts depth frames, applies spatial
    /// filters and plots modified prames
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_spatial_filter() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_spatial_filter_block(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates Sync processing block. This block accepts arbitrary frames and output composite frames
    /// of best matches Some frames may be released within the syncer if they are waiting for match for
    /// too long Syncronization is done (mostly) based on timestamps so good hardware timestamps are a pre-condition
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_sync_processing() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_sync_processing_block(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates Depth post-processing filter block. This block accepts depth frames, applies temporal filter
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_temporal_filter() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_temporal_filter_block(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates depth thresholding processing block By controlling min and max options on the block, one could
    /// filter out depth values that are either too large or too small, as a software post-processing step
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_threshold() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_threshold(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates depth units transformation processing block All of the pixels are transformed from depth
    /// units into meters.
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_units_transform() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_units_transform(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates YUY decoder processing block. This block accepts raw YUY frames and outputs frames of other
    /// formats. YUY is a common video format used by a variety of web-cams. It benefits from packing pixels
    /// into 2 bytes per pixel without signficant quality drop. YUY representation can be converted back to more
    /// usable RGB form, but this requires somewhat costly conversion. The SDK will automatically try to use SSE2
    /// and AVX instructions and CUDA where available to get best performance. Other implementations (using GLSL,
    /// OpenCL, Neon and NCS) should follow.
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_yuy_decoder() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_yuy_decoder(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }

    /// Creates Depth post-processing zero order fix block. The filter invalidates pixels that
    /// has a wrong value due to zero order effect
    ///
    /// # Returns
    /// * `Ok(ProcessingBlock)` on success.
    /// * `Err(Error)` on failure.
    pub fn create_zero_order_invalidation() -> Result<Self, Error> {
        let mut error = Error::default();
        let processing_block = ProcessingBlock {
            handle: unsafe { rs2::rs2_create_zero_order_invalidation_block(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(processing_block)
        }
    }
}
