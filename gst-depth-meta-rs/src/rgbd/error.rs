use gst::glib;
use once_cell::sync::Lazy;
use std::borrow::Cow;

/// RvlError represents all possible errors that may occur during RVL encoding in GStreamer.
/// It can convert into a multitude of GStreamer errors, in the attempt of making it convenient
/// to use, while requiring as little boilerplate as possible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RgbdError {
    /// No video info is available et, but an operation that required video info have been attempted.
    NoVideoInfo,
    /// GStreamer error: The video/rgbd caps are missing a field with the given name.
    MissingCapsField(String),
    /// GStreamer error: The field with the given `name` of the video/rgbd CAPS is of incorrect
    /// type. It should be of type `type_`.
    WrongCapsFormat {
        /// The name of the field.
        name: String,
        /// The name of the type we expected.
        type_: &'static str,
    },
    /// The input buffer is not properly aligned to contain depth video.
    BufferNotAligned,
}
impl std::error::Error for RgbdError {}
impl std::fmt::Display for RgbdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            RgbdError::NoVideoInfo => {
                "RGBD: No VideoInfo. Are we missing a CAPS negotiation?".to_string()
            }
            RgbdError::MissingCapsField(field) => format!("RGBD: Missing caps field `{}`", field),
            RgbdError::WrongCapsFormat {
                name: field,
                type_: expected,
            } => format!(
                "RGBD: CAPS field `{}` has wrong format. Expected {}",
                field, expected
            ),
            RgbdError::BufferNotAligned => "Input buffer is not aligned to u16".to_string(),
        };

        write!(f, "{}", msg)
    }
}
impl From<RgbdError> for gst::FlowError {
    fn from(_: RgbdError) -> Self {
        gst::FlowError::Error
    }
}
impl From<RgbdError> for glib::BoolError {
    fn from(e: RgbdError) -> Self {
        glib::BoolError::new(Cow::from(e.to_string()), "?", "?", 0)
    }
}

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "rgbd",
        gst::DebugColorFlags::empty(),
        Some("RGBD convenience functions"),
    )
});

impl From<RgbdError> for gst::LoggableError {
    fn from(e: RgbdError) -> Self {
        gst::LoggableError::new(*CAT, glib::BoolError::from(e))
    }
}
