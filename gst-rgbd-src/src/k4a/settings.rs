use super::enums::*;
use super::error::*;
use super::streams::*;
use libk4a::{ColorResolution, DepthMode, DeviceConfiguration, ImageFormat};
use std::convert::{From, TryFrom};

// Streams enabled by default
/// Determines whether streaming depth frames is enabled by default.
pub(crate) const DEFAULT_ENABLE_DEPTH: bool = true;
/// Determines whether streaming IR frames is enabled by default.
pub(crate) const DEFAULT_ENABLE_IR: bool = false;
/// Determines whether streaming color frames is enabled by default.
pub(crate) const DEFAULT_ENABLE_COLOR: bool = false;
/// Determines whether streaming IMU measurements is enabled by default.
pub(crate) const DEFAULT_ENABLE_IMU: bool = false;

// Default settings for K4A specifics
/// The format utilised for streaming depth frames from K4A device.
pub(crate) const DEPTH_FORMAT: ImageFormat = ImageFormat::K4A_IMAGE_FORMAT_DEPTH16;
/// The format utilised for streaming IR frames from K4A device.
pub(crate) const IR_FORMAT: ImageFormat = ImageFormat::K4A_IMAGE_FORMAT_IR16;
/// Default color format for streaming from K4A device.
pub(crate) const DEFAULT_COLOR_FORMAT: K4aColorFormat = K4aColorFormat::NV12;
/// Default color resolution for streming from K4A device.
pub(crate) const DEFAULT_COLOR_RESOLUTION: K4aColorResolution = K4aColorResolution::C720p;
/// Default depth mode for streming from K4A device.
pub(crate) const DEFAULT_DEPTH_MODE: K4aDepthMode = K4aDepthMode::NfovUnbinned;
/// Default behaviour for rectifying depth frames.
pub(crate) const DEFAULT_RECTIFY_DEPTH: bool = false;
/// Default behaviour for attaching camera meta buffers.
pub(crate) const DEFAULT_ATTACH_CAMERA_META: bool = false;

// Framerates
/// The rate at which IMU outputs its measurements.
pub(crate) const IMU_SAMPLING_RATE_HZ: i32 = 208;
/// Default framerate for streaming video from K4A device.
pub(crate) const DEFAULT_FRAMERATE: K4aFramerate = K4aFramerate::FPS15;

// Default settings for GStreamer specifics
/// Default timeout duration while waiting for frames when streaming from K4A device.
pub(crate) const DEFAULT_GET_CAPTURE_TIMEOUT: i32 = 1000;
/// Default behaviour of looping playback from recording.
pub(crate) const DEFAULT_LOOP_RECORDING: bool = false;
/// Default behaviour for liveliness of the element when playing back from a recording.
pub(crate) const DEFAULT_REAL_TIME_PLAYBACK: bool = false;

// TODO: If desired, make these into properties with the appropriate support
pub(crate) const DEPTH_DELAY_OFF_COLOR_USEC: i32 = 0;
pub(crate) const WIRED_SYNCH_MODE: libk4a::WiredSyncMode =
    libk4a::WiredSyncMode::K4A_WIRED_SYNC_MODE_STANDALONE;
pub(crate) const SUBORDINATE_DELAY_OFF_MASTER_USEC: u32 = 0;
pub(crate) const DISABLE_STREAMING_INDICATOR: bool = false;

/// A struct containing properties.
pub(crate) struct Settings {
    pub(crate) device_settings: DeviceSettings,
    pub(crate) playback_settings: PlaybackSettings,
    pub(crate) desired_streams: EnabledStreams,
    pub(crate) rectify_depth: bool,
    pub(crate) attach_camera_meta: bool,
}

/// A struct containing properties specific for streaming from a physical K4A device.
pub(crate) struct DeviceSettings {
    pub(crate) serial: String,
    pub(crate) color_format: K4aColorFormat,
    pub(crate) color_resolution: K4aColorResolution,
    pub(crate) depth_mode: K4aDepthMode,
    pub(crate) framerate: K4aFramerate,
    pub(crate) get_capture_timeout: i32,
}

/// A struct containing properties specific for streaming playback from a recording.
pub(crate) struct PlaybackSettings {
    pub(crate) recording_location: String,
    pub(crate) loop_recording: bool,
    pub(crate) real_time_playback: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            device_settings: DeviceSettings {
                serial: String::default(),
                color_format: DEFAULT_COLOR_FORMAT,
                color_resolution: DEFAULT_COLOR_RESOLUTION,
                depth_mode: DEFAULT_DEPTH_MODE,
                framerate: DEFAULT_FRAMERATE,
                get_capture_timeout: DEFAULT_GET_CAPTURE_TIMEOUT,
            },
            desired_streams: EnabledStreams::default(),
            playback_settings: PlaybackSettings {
                recording_location: String::default(),
                loop_recording: DEFAULT_LOOP_RECORDING,
                real_time_playback: DEFAULT_REAL_TIME_PLAYBACK,
            },
            rectify_depth: DEFAULT_RECTIFY_DEPTH,
            attach_camera_meta: DEFAULT_ATTACH_CAMERA_META,
        }
    }
}

/// Determines the applicable `DeviceConfiguration` based on the selected settings.
impl TryFrom<&Settings> for DeviceConfiguration {
    type Error = K4aSrcError;
    fn try_from(settings: &Settings) -> Result<Self, Self::Error> {
        let device_settings = &settings.device_settings;

        // TODO: If desired, implement possibility of not having the streams synchronised (requires quite a lot of work)
        // Synchronisation is allowed only if both cameras are enabled
        let synchronised_images_only = (settings.desired_streams.depth
            || settings.desired_streams.ir)
            && settings.desired_streams.color;

        // Create `DeviceConfiguration` based on settings
        Ok(DeviceConfiguration {
            color_format: libk4a::ImageFormat::from(device_settings.color_format),
            color_resolution: ColorResolution::from(settings),
            depth_mode: DepthMode::from(settings),
            camera_fps: libk4a::Fps::from(device_settings.framerate),
            synchronized_images_only: synchronised_images_only,
            depth_delay_off_color_usec: DEPTH_DELAY_OFF_COLOR_USEC,
            wired_sync_mode: WIRED_SYNCH_MODE,
            subordinate_delay_off_master_usec: SUBORDINATE_DELAY_OFF_MASTER_USEC,
            disable_streaming_indicator: DISABLE_STREAMING_INDICATOR,
        })
    }
}

/// Determines the applicable `DepthMode` while taking into account what streams are
/// enabled, if any. Used when converting to `DeviceConfiguration`.
impl From<&Settings> for DepthMode {
    fn from(settings: &Settings) -> DepthMode {
        if settings.desired_streams.depth {
            // If depth is enabled, use `depth-mode` property
            DepthMode::from(settings.device_settings.depth_mode)
        } else if settings.desired_streams.ir {
            // If IR is enabled without depth, use `K4A_DEPTH_MODE_PASSIVE_IR`
            DepthMode::K4A_DEPTH_MODE_PASSIVE_IR
        } else {
            // If neither depth or IR is enabled, use `K4A_DEPTH_MODE_OFF`
            DepthMode::K4A_DEPTH_MODE_OFF
        }
    }
}

/// Determines the applicable `ColorResolution` while taking into account whether
/// the color stream is enabled or not. Used when converting to `DeviceConfiguration`.
impl From<&Settings> for ColorResolution {
    fn from(settings: &Settings) -> ColorResolution {
        if settings.desired_streams.color {
            // If color is enabled, use `color-resolution` property
            ColorResolution::from(settings.device_settings.color_resolution)
        } else {
            // If color is disabled, use `K4A_COLOR_RESOLUTION_OFF`
            ColorResolution::K4A_COLOR_RESOLUTION_OFF
        }
    }
}
