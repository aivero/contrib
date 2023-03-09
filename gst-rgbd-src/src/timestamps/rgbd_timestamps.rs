use super::timestamp_internals::TimestampInternals;
use super::timestamp_mode::TimestampMode;
use gst::prelude::ElementExtManual;
use gst_base::prelude::BaseSrcExt;
use gst_base::{subclass::prelude::*, *};
use std::sync::{Arc, Mutex};

/// A trait that implements timestamping for source GStreamer elements, based on BaseSrc, that output `video/rgbd` stream.
///
/// # Implementation
/// * Only `self.get_timestamp_internals()` has to be implemented.
/// * If desired, default implementations of other functions can be overriden. However, this should not be necessary.
///
/// # Usage
/// This is the general guidline for usage of this trait. Special scenarios might require deviation from this recommendation.
/// ## Setup
/// * Implement `self.get_timestamp_internals()`.
/// * Use `self.set_timestamp_mode()`, e.g. in element's `set_property()` implementation.
/// * Use `self.set_buffer_duration()` after framerate has been negoriated, e.g. in element's `fixate()` implementation.
/// ## Timestamping
/// * Use `self.set_rgbd_timestamp()` on each outgoing buffer, e.g. main and all auxiliary buffers.
pub trait RgbdTimestamps: BaseSrcImpl {
    /// Get a thread-safe, mutex-protected pointer to `TimestampInternals` that `RgbdTimestamps` trait requires to function.
    /// Currently, the best way is to include `Arc<Mutex<TimestampInternals>>` as a field in the implementor struct and have
    /// implementation of this function return a `.clone()` of it.
    fn get_timestamp_internals(&self) -> Arc<Mutex<TimestampInternals>>;

    /// Set the timestamp mode that `RgbdTimestamps` trait should utilise.
    ///
    /// # Arguments
    /// * `base_src` - Element utilising the trait.
    /// * `timestamp_mode` - Mode that the trait should be configure for.
    ///
    /// # Default
    /// * Sets the `timestamp_mode` and enables `do-timestamp` if TimestampMode::ClockMain is selected.
    fn set_timestamp_mode(&self, base_src: &gst_base::BaseSrc, timestamp_mode: TimestampMode) {
        // Set the timestamp mode
        self.get_timestamp_internals()
            .lock()
            .unwrap()
            .timestamp_mode = timestamp_mode;

        // Enables `do-timestamp` if TimestampMode::ClockMain is selected, else disable.
        base_src.set_do_timestamp(matches!(timestamp_mode, TimestampMode::ClockMain));
    }

    /// Set the buffer duration that `RgbdTimestamps` trait should apply to each buffer based on the negotiated framerate.
    ///
    /// # Arguments
    /// * `framerate` - Negotiated framerate.
    ///
    /// # Default
    /// * Sets the `buffer_duration` as inverse of `framerate`.
    fn set_buffer_duration(&self, framerate: f32) {
        self.get_timestamp_internals()
            .lock()
            .unwrap()
            .buffer_duration = gst::ClockTime::from_nseconds(
            std::time::Duration::from_secs_f32(1.0 / framerate).as_nanos() as u64,
        );
    }

    /// Set the appropriate timestamp and duration to the `buffer`.
    ///
    /// # Arguments
    /// * `base_src` - Element utilising the trait.
    /// * `buffer` - Buffer that should be modified.
    /// * `is_buffer_main` - A flag that determines whether the buffer is main or not. Set to *false* for auxiliary streams.
    /// * `camera_timestamp` - Timestamp of the buffer acquired from the utilised camera. Set to `gst::CLOCK_TIME_NONE`
    /// if unavailable or neither of the camera based timestamps is desired.
    ///
    /// # Default
    /// * Utilises `self.determine_timestamp()` and sets `pts`, `dts` and `duration` of the buffer.
    fn set_rgbd_timestamp(
        &self,
        base_src: &gst_base::BaseSrc,
        buffer: &mut gst::BufferRef,
        is_buffer_main: bool,
        camera_timestamp: gst::ClockTime,
    ) {
        // Determine timestamp based on the selected mode
        let timestamp = self.determine_timestamp(base_src, is_buffer_main, camera_timestamp);

        // Set timestamps
        buffer.set_pts(timestamp);
        buffer.set_dts(timestamp);

        // Set duration
        buffer.set_duration(
            self.get_timestamp_internals()
                .lock()
                .unwrap()
                .buffer_duration,
        );
    }

    /// Determine the timestamp to use for a buffer based on the selected `timestamp-mode`.
    ///
    /// # Arguments
    /// * `base_src` - Element utilising the trait.
    /// * `is_buffer_main` - A flag that determines whether the buffer is main or not. Set to *false* for auxiliary streams.
    /// * `camera_timestamp` - Timestamp of the buffer acquired from the utilised camera. Set to `gst::CLOCK_TIME_NONE`
    /// if unavailable or neither of the camera based timestamps is desired.
    ///
    /// # Returns
    /// * `gst::ClockTime` containing the timestamp or gst::CLOCK_TIME_NONE if the selected mode does not require timestamp.
    ///
    /// # Default
    /// * Returns gst::CLOCK_TIME_NONE for `TimestampMode::Ignore` or `TimestampMode::ClockMain`.
    /// * Returns output of `self.determine_timestamp_clock_all()` for TimestampMode::ClockAll.
    /// * Returns output of `self.determine_timestamp_camera_common()` for TimestampMode::CameraCommon.
    /// * Returns output of `self.determine_timestamp_camera_individual()` for TimestampMode::CameraIndividual.
    fn determine_timestamp(
        &self,
        base_src: &gst_base::BaseSrc,
        is_buffer_main: bool,
        camera_timestamp: gst::ClockTime,
    ) -> gst::ClockTime {
        // Get the timestamp mode
        let timestamp_mode = self
            .get_timestamp_internals()
            .lock()
            .unwrap()
            .timestamp_mode;

        // Proceed based on the timestamp mode
        use TimestampMode::*;
        match timestamp_mode {
            Ignore | ClockMain => {
                //     Variant `TimestampMode::Ignore` does not require timestamps
                //     Variant `TimestampMode::ClockMain` is handled by the parent class
                gst::ClockTime::ZERO
            }
            ClockAll => {
                // Variant `TimestampMode::ClockAll`
                self.determine_timestamp_clock_all(base_src, is_buffer_main)
            }
            CameraCommon => {
                // Variant `TimestampMode::CameraCommon`
                self.determine_timestamp_camera_common(base_src, is_buffer_main, camera_timestamp)
            }
            CameraIndividual => {
                // Variant `TimestampMode::CameraIndividual`
                self.determine_timestamp_camera_individual(
                    base_src,
                    is_buffer_main,
                    camera_timestamp,
                )
            }
            FrameCounting => {
                // Variant `TimestampMode::FrameCounting`
                self.determine_timestamp_frame_counting(base_src, is_buffer_main)
            }
        }
    }

    /// Determine the `TimestampMode::ClockAll` timestamp to use for a buffer.
    ///
    /// # Arguments
    /// * `base_src` - Element utilising the trait.
    /// * `is_buffer_main` - A flag that determines whether the buffer is main or not. Set to *false* for auxiliary streams.
    ///
    /// # Returns
    /// * `gst::ClockTime` containing the timestamp.
    ///
    /// # Default
    /// * See in-line documentation in the source code.
    fn determine_timestamp_clock_all(
        &self,
        base_src: &gst_base::BaseSrc,
        is_buffer_main: bool,
    ) -> gst::ClockTime {
        // Get mutable reference to timestamp internals
        let timestamp_internals = self.get_timestamp_internals();
        let timestamp_internals = &mut *timestamp_internals.lock().unwrap();

        // Determine the common timestamp (computed only once for the main buffer)
        if is_buffer_main {
            // This mode utilises the current running time to determine the timestamps
            let running_time = base_src
                .current_running_time()
                .unwrap_or(gst::ClockTime::ZERO);

            // The timestamp is also based on whether the element is live or not
            if base_src.is_live() {
                // Use current running time for live mode
                timestamp_internals.frameset_common_timestamp = running_time;
            } else {
                // Non-live mode must have the first buffer with timestamp of 0
                if timestamp_internals.stream_start_offset == gst::ClockTime::ZERO {
                    // Compute first frame timestamp based on running time, if not yet defined
                    timestamp_internals.stream_start_offset = running_time;
                    // Set the timestamp of the first frameset to 0
                    timestamp_internals.frameset_common_timestamp = gst::ClockTime::ZERO;
                } else {
                    // Once first frame offset is initialised, subtract from it the current running time
                    timestamp_internals.frameset_common_timestamp =
                        running_time - timestamp_internals.stream_start_offset;
                }
            }
        }

        // Return the common timestamp (for all buffers)
        if timestamp_internals.frameset_common_timestamp == gst::ClockTime::ZERO {
            // Return 0 if clock is not yet initialised - invalid timestamp (gst::CLOCK_TIME_NONE)
            gst::ClockTime::from_nseconds(0)
        } else {
            // Else just return the valid timestamp
            timestamp_internals.frameset_common_timestamp
        }
    }

    /// Determine the `TimestampMode::CameraCommon` timestamp to use for a buffer.
    ///
    /// # Arguments
    /// * `base_src` - Element utilising the trait.
    /// * `is_buffer_main` - A flag that determines whether the buffer is main or not. Set to *false* for auxiliary streams.
    /// * `camera_timestamp` - Timestamp of the buffer acquired from the utilised camera. Set to `gst::CLOCK_TIME_NONE`
    /// if unavailable or neither of the camera based timestamps is desired.
    ///
    /// # Returns
    /// * `gst::ClockTime` containing the timestamp.
    ///
    /// # Default
    /// * See in-line documentation in the source code.
    fn determine_timestamp_camera_common(
        &self,
        base_src: &gst_base::BaseSrc,
        is_buffer_main: bool,
        camera_timestamp: gst::ClockTime,
    ) -> gst::ClockTime {
        // Get mutable reference to timestamp internals
        let timestamp_internals = self.get_timestamp_internals();
        let timestamp_internals = &mut *timestamp_internals.lock().unwrap();

        // Determine common timestamp (computed only once for the main buffer)
        if is_buffer_main {
            if timestamp_internals.stream_start_offset == gst::ClockTime::ZERO {
                // Compute first frame timestamp, if not yet defined
                if base_src.is_live() {
                    // For live mode, get the offset of the first frame based on the camera timestamp and current running time
                    // Make sure the timestamp is positive as gst::ClockTime uses u64 internally
                    // Furthermore, assign `is_camera_ahead_of_gstreamer` for future reference
                    let running_time = base_src
                        .current_running_time()
                        .unwrap_or(gst::ClockTime::ZERO);
                    if camera_timestamp > running_time {
                        timestamp_internals.stream_start_offset = camera_timestamp - running_time;
                        timestamp_internals.is_camera_ahead_of_gstreamer = true;
                    } else {
                        timestamp_internals.stream_start_offset = running_time - camera_timestamp;
                        timestamp_internals.is_camera_ahead_of_gstreamer = false;
                    }
                } else {
                    // For non-live mode, get the offset of the first frame based only on the
                    // camera timestamp, so that the first buffer has timestamp of 0
                    timestamp_internals.stream_start_offset = camera_timestamp;
                    timestamp_internals.is_camera_ahead_of_gstreamer = true;
                }
                // Set the timestamp of the first frameset to 0
                timestamp_internals.frameset_common_timestamp = gst::ClockTime::from_nseconds(0);
            } else {
                // Once first frame offset is initialised, use the offset to provide adequate timestamp
                if timestamp_internals.is_camera_ahead_of_gstreamer {
                    // Subtract if camera clock domain is ahead of GStreamer
                    timestamp_internals.frameset_common_timestamp =
                        camera_timestamp - timestamp_internals.stream_start_offset;
                } else {
                    // Add otherwise
                    timestamp_internals.frameset_common_timestamp =
                        camera_timestamp + timestamp_internals.stream_start_offset;
                }
            }
        }

        timestamp_internals.frameset_common_timestamp
    }

    /// Determine the `TimestampMode::CameraIndividual` timestamp to use for a buffer.
    ///
    /// # Arguments
    /// * `base_src` - Element utilising the trait.
    /// * `is_buffer_main` - A flag that determines whether the buffer is main or not. Set to *false* for auxiliary streams.
    /// * `camera_timestamp` - Timestamp of the buffer acquired from the utilised camera. Set to `gst::CLOCK_TIME_NONE`
    /// if unavailable or neither of the camera based timestamps is desired.
    ///
    /// # Returns
    /// * `gst::ClockTime` containing the timestamp.
    ///
    /// # Default
    /// * See in-line documentation in the source code.
    fn determine_timestamp_camera_individual(
        &self,
        base_src: &gst_base::BaseSrc,
        is_buffer_main: bool,
        camera_timestamp: gst::ClockTime,
    ) -> gst::ClockTime {
        // Get mutable reference to timestamp internals
        let timestamp_internals = self.get_timestamp_internals();
        let timestamp_internals = &mut *timestamp_internals.lock().unwrap();

        // Use timestamp of the main buffer if we are dealing with meta streams that do not have timestamps
        if camera_timestamp == gst::ClockTime::ZERO {
            return timestamp_internals.frameset_common_timestamp;
        }

        // Compute first frame timestamp, if not yet defined
        if timestamp_internals.stream_start_offset == gst::ClockTime::ZERO {
            if base_src.is_live() {
                // For live mode, get the offset of the first frame based on the camera timestamp and current running time
                // Make sure the timestamp is positive as gst::ClockTime uses u64 internally
                // Furthermore, assign `is_camera_ahead_of_gstreamer` for future reference
                let running_time = base_src
                    .current_running_time()
                    .unwrap_or(gst::ClockTime::ZERO);
                if camera_timestamp > running_time {
                    timestamp_internals.stream_start_offset = camera_timestamp - running_time;
                    timestamp_internals.is_camera_ahead_of_gstreamer = true;
                } else {
                    timestamp_internals.stream_start_offset = running_time - camera_timestamp;
                    timestamp_internals.is_camera_ahead_of_gstreamer = false;
                }
            } else {
                // For non-line mode, get the offset of the first frame based only on the camera timestamp, so that the first buffer has timestamp of 0
                timestamp_internals.stream_start_offset = camera_timestamp;
            }
        }

        // Update the common timestamp when processing the main buffer
        // The common timestamp is used for meta streams that do not have camera timestamps
        if is_buffer_main {
            // Once first frame offset is initialised, use the offset to provide adequate timestamp
            if timestamp_internals.is_camera_ahead_of_gstreamer {
                // Subtract if camera clock domain is ahead of GStreamer
                timestamp_internals.frameset_common_timestamp =
                    camera_timestamp - timestamp_internals.stream_start_offset;
            } else {
                // Add otherwise
                timestamp_internals.frameset_common_timestamp =
                    camera_timestamp + timestamp_internals.stream_start_offset;
            }
            return timestamp_internals.frameset_common_timestamp;
        }

        // Once first frame offset is initialised, use the offset to provide adequate timestamp
        if timestamp_internals.is_camera_ahead_of_gstreamer {
            // Subtract if camera clock domain is ahead of GStreamer
            camera_timestamp - timestamp_internals.stream_start_offset
        } else {
            // Add otherwise
            camera_timestamp + timestamp_internals.stream_start_offset
        }
    }

    /// Determine the `TimestampMode::FrameCounting` timestamp to use for a buffer.
    ///
    /// # Arguments
    /// * `base_src` - Element utilising the trait.
    /// * `is_buffer_main` - A flag that determines whether the buffer is main or not. Set to *false* for auxiliary streams.
    ///
    /// # Returns
    /// * `gst::ClockTime` containing the timestamp.
    fn determine_timestamp_frame_counting(
        &self,
        base_src: &gst_base::BaseSrc,
        is_buffer_main: bool,
    ) -> gst::ClockTime {
        // Get mutable reference to timestamp internals
        let timestamp_internals = self.get_timestamp_internals();
        let timestamp_internals = &mut *timestamp_internals.lock().unwrap();

        // Compute the appropriate timestamp and increment the sequence number (only on main buffers)
        if is_buffer_main {
            timestamp_internals.frameset_common_timestamp =
                timestamp_internals.sequence_number * timestamp_internals.buffer_duration;
            timestamp_internals.sequence_number += 1;
        }

        // Return the timestamp
        if base_src.is_live() {
            // For live mode, we must offset all timestamps based on the running time of the first frame
            if timestamp_internals.stream_start_offset == gst::ClockTime::ZERO {
                timestamp_internals.stream_start_offset = base_src
                    .current_running_time()
                    .unwrap_or(gst::ClockTime::ZERO);
            }
            timestamp_internals.stream_start_offset + timestamp_internals.frameset_common_timestamp
        } else {
            // For non-live mode, no offset is needed as start from 0
            timestamp_internals.frameset_common_timestamp
        }
    }
}
