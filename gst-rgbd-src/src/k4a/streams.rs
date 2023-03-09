use super::settings::*;

// Unique identifiers of streams
/// ID of the depth stream.
pub(crate) const STREAM_ID_DEPTH: &str = "depth";
/// ID of the IR stream.
pub(crate) const STREAM_ID_IR: &str = "ir";
/// ID of the color stream.
pub(crate) const STREAM_ID_COLOR: &str = "color";
/// ID of the IMU stream.
pub(crate) const STREAM_ID_IMU: &str = "imu";
/// ID of the camera meta stream.
pub(crate) const STREAM_ID_CAMERAMETA: &str = "camerameta";

/// A struct containing information about what streams are enabled.
#[derive(Clone, Copy)]
pub(crate) struct EnabledStreams {
    /// Determines whether depth stream is enabled.
    pub(crate) depth: bool,
    /// Determines whether IR stream is enabled.
    pub(crate) ir: bool,
    /// Determines whether color stream is enabled.
    pub(crate) color: bool,
    /// Determines whether IMU stream is enabled.
    pub(crate) imu: bool,
}

impl Default for EnabledStreams {
    fn default() -> Self {
        Self {
            depth: DEFAULT_ENABLE_DEPTH,
            ir: DEFAULT_ENABLE_IR,
            color: DEFAULT_ENABLE_COLOR,
            imu: DEFAULT_ENABLE_IMU,
        }
    }
}

impl EnabledStreams {
    /// Determine whether at least one video stream is enabled. IMU stream is ignored.
    ///
    /// # Returns
    /// * `true` if at least one video stream is enabled.
    /// * `false` if no stream is enabled.
    pub(crate) fn is_any_video_enabled(self) -> bool {
        self.depth | self.ir | self.color
    }

    /// Determine whether there are any conflict between `enabled_streams` and
    /// `available_streams`
    ///
    /// # Arguments
    /// * `enabled_streams` - The streams that are enabled.
    /// * `available_streams` - The streams that are available.
    ///
    /// # Returns
    /// * `Vec<&str>` of conflicting streams, which is empty if there is no conflict.
    pub(crate) fn are_streams_available(
        enabled_streams: EnabledStreams,
        available_streams: EnabledStreams,
    ) -> bool {
        (available_streams.imu || !enabled_streams.imu)
            && (available_streams.color || !enabled_streams.color)
            && (available_streams.ir || !enabled_streams.ir)
            && (available_streams.depth || !enabled_streams.depth)
    }
}

/// An Id for distinguishing between streams when iterating over `Streams`.
#[derive(PartialEq)]
pub(crate) enum StreamId {
    Depth,
    Ir,
    Color,
    Imu,
}

impl StreamId {
    pub(crate) fn get_string(&self) -> &'static str {
        match self {
            StreamId::Depth => STREAM_ID_DEPTH,
            StreamId::Ir => STREAM_ID_IR,
            StreamId::Color => STREAM_ID_COLOR,
            StreamId::Imu => STREAM_ID_IMU,
        }
    }
}

/// An array for storing all streams.
pub(crate) type Streams = [Stream; 4];

/// A struct with fields describing the properties of a stream.
pub(crate) struct Stream {
    /// Is the stream enabled?
    pub(crate) enabled: bool,

    /// If the stream the main stream. This is true if the stream
    /// is the first enabled stream. When `enabled` is `false`, this
    /// can never be `true`.
    pub(crate) is_main: bool,

    /// The streams id.
    pub(crate) id: StreamId,
}

impl From<EnabledStreams> for Streams {
    fn from(enabled_streams: EnabledStreams) -> Self {
        [
            Stream {
                enabled: enabled_streams.depth,
                is_main: enabled_streams.depth,
                id: StreamId::Depth,
            },
            Stream {
                enabled: enabled_streams.ir,
                is_main: !enabled_streams.depth && enabled_streams.ir,
                id: StreamId::Ir,
            },
            Stream {
                enabled: enabled_streams.color,
                is_main: !enabled_streams.depth && !enabled_streams.ir && enabled_streams.color,
                id: StreamId::Color,
            },
            Stream {
                enabled: enabled_streams.imu,
                is_main: !enabled_streams.depth
                    && !enabled_streams.ir
                    && !enabled_streams.color
                    && enabled_streams.imu,
                id: StreamId::Imu,
            },
        ]
    }
}
