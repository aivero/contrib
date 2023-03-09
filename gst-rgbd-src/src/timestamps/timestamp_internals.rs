use super::timestamp_mode::TimestampMode;

/// A struct that contains data associated with timestamps.
#[derive(Debug, Clone, Copy)]
pub struct TimestampInternals {
    /// Timestamp mode that determines the timestamps of outgoing buffers.
    pub timestamp_mode: TimestampMode,
    /// Contains offset of the first buffer.
    pub stream_start_offset: gst::ClockTime,
    /// Contains common timestamp for a single capture.
    pub frameset_common_timestamp: gst::ClockTime,
    /// Contains duration of each buffer.
    pub buffer_duration: gst::ClockTime,
    /// A flag that determines whether camera clock is ahead of GStreamer clock at the beginning of streaming.
    /// If true, the extracted camera timestamps have a value that is larger than the running time of the
    /// GStreamer pipeline. This flag is used to determine whether to add or subtract the stream start offset
    /// from camera timestamps.
    pub is_camera_ahead_of_gstreamer: bool,
    /// The sequence number of the current frameset being processed. This is used for FrameCounting timestamp
    /// mode.
    pub sequence_number: u64,
}

/// Implentation of Default trait for TimestampInternals.
impl Default for TimestampInternals {
    fn default() -> Self {
        Self {
            buffer_duration: gst::ClockTime::ZERO,
            frameset_common_timestamp: gst::ClockTime::ZERO,
            stream_start_offset: gst::ClockTime::ZERO,
            timestamp_mode: TimestampMode::default(),
            is_camera_ahead_of_gstreamer: bool::default(),
            sequence_number: 0,
        }
    }
}
