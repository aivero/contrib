// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.
use crate::error::Error;
use crate::pipeline::Pipeline;
use crate::pipeline_profile::PipelineProfile;

// Expose `rs2_format` and `rs2_stream` for external use.
pub use rs2::rs2_format;
pub use rs2::rs2_stream;

/// Struct representation of configuration [`Config`](../config/struct.Config.html)
/// that wraps around `rs2_config` handle. The [`Config`](../config/struct.Config.html)
/// allows, in combination with [`Pipeline`](../pipeline/struct.Pipeline.html), to
/// request filters for the streams and [`Device`](../device/struct.Device.html)
/// selection and configuration.
pub struct Config(pub(crate) *mut rs2::rs2_config);

/// Safe releasing of the `rs2_config` handle.
impl Drop for Config {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_config(self.0) }
    }
}

impl From<*mut rs2::rs2_config> for Config {
    fn from(c: *mut rs2::rs2_config) -> Self {
        Config(c)
    }
}

impl Config {
    /// Create a [`Config`](../config/struct.Config.html) instance. The
    /// [`Config`](../config/struct.Config.html) allows
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) users to request filters for the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) streams and
    /// [`Device`](../device/struct.Device.html) selection and configuration. This is
    /// an optional step in [`Pipeline`](../pipeline/struct.Pipeline.html) creation, as
    /// the [`Pipeline`](../pipeline/struct.Pipeline.html) resolves its streaming
    /// [`Device`](../device/struct.Device.html) internally.
    /// [`Config`](../config/struct.Config.html) provides its users a way to set the
    /// filters and test if there is no conflict with the pipeline requirements from the
    /// [`Device`](../device/struct.Device.html). It also allows the user to find a
    /// matching [`Device`](../device/struct.Device.html) for the config filters and
    /// the [`Pipeline`](../pipeline/struct.Pipeline.html), in order to select a
    /// [`Device`](../device/struct.Device.html) explicitly, and modify its controls
    /// before streaming starts.
    ///
    /// # Returns
    /// * `Ok(Config)` on success.
    /// * `Err(Error)` on failure.
    pub fn create() -> Result<Config, Error> {
        Error::call0(rs2::rs2_create_config)
    }

    /// Enable a [`Device`](../device/struct.Device.html) stream explicitly, with
    /// selected parameters. The method allows the application to request a stream with specific
    /// configuration. If no stream is explicitly enabled, the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) configures the
    /// [`Device`](../device/struct.Device.html) and its streams according to the
    /// attached computer vision modules and processing blocks requirements, or default
    /// configuration for the first available
    /// [`Device`](../device/struct.Device.html). The application can configure any of
    /// the input stream parameters according to its requirement, or set to 0 for don't care value.
    /// The [`Config`](../config/struct.Config.html) accumulates the application calls
    /// for enable configuration methods, until the configuration is applied. Multiple enable
    /// stream calls for the same stream with conflicting parameters override each other, and the
    /// last call is maintained. Upon calling
    /// [`Config::resolve()`](../config/struct.Config.html#method.resolve), the
    /// [`Config`](../config/struct.Config.html) checks for conflicts between the
    /// application configuration requests and the attached computer vision modules and processing
    /// blocks requirements, and fails if conflicts are found. Before
    /// [`Config::resolve()`](../config/struct.Config.html#method.resolve) is called,
    /// no conflict check is done.
    ///
    /// # Arguments
    /// * `stream` - Stream type to be enabled.
    /// * `index` - Stream index, used for multiple streams of the same type. -1 indicates any.
    /// * `width` - Stream image width - for images streams. 0 indicates any.
    /// * `height` - Stream image height - for images streams. 0 indicates any.
    /// * `format` - Stream data format - pixel format for images streams, of data type for other
    /// streams. `RS2_FORMAT_ANY` indicates any.
    /// * `framerate` - Stream frames per second. 0 indicates any.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_stream(
        &self,
        stream: rs2_stream,
        index: i32,
        width: i32,
        height: i32,
        format: rs2_format,
        framerate: i32,
    ) -> Result<(), Error> {
        Error::call7(
            rs2::rs2_config_enable_stream,
            self.0,
            stream,
            index,
            width,
            height,
            format,
            framerate,
        )
    }

    /// Enable all [`Device`](../device/struct.Device.html) streams explicitly. The
    /// conditions and behavior of this method are similar to those of
    /// [`Config::enable_stream()`](../config/struct.Config.html#method.enable_stream).
    /// This filter enables all raw `Streams` of the selected
    /// [`Device`](../device/struct.Device.html). The
    /// [`Device`](../device/struct.Device.html) is either selected explicitly by the
    /// application, or by the [`Pipeline`](../pipeline/struct.Pipeline.html)
    /// requirements or default. The list of streams is
    /// [`Device`](../device/struct.Device.html) dependent.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_all_streams(&self) -> Result<(), Error> {
        Error::call1(rs2::rs2_config_enable_all_stream, self.0)
    }

    /// Select a specific [`Device`](../device/struct.Device.html) explicitly by its
    /// `serial` number, to be used by the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html).The conditions and behavior of
    /// this method are similar to those of
    /// [`Config::enable_stream()`](../config/struct.Config.html#method.enable_stream).
    /// This method is required if the application needs to set
    /// [`Device`](../device/struct.Device.html) or
    /// [`Sensor`](../sensor/struct.Sensor.html) settings prior to
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) streaming, to enforce the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) to use the configured
    /// [`Device`](../device/struct.Device.html).
    ///
    /// # Arguments
    /// * `serial` - [`Device`](../device/struct.Device.html) serial number, as
    /// returned by `RS2_CAMERA_INFO_SERIAL_NUMBER`.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_device(&self, serial: &str) -> Result<(), Error> {
        let s = std::ffi::CString::new(serial).expect("Failed to create CString");
        Error::call2(rs2::rs2_config_enable_device, self.0, s.as_ptr())
    }

    /// Select a recorded [`Device`](../device/struct.Device.html) from a `file`, to be
    /// used by the [`Pipeline`](../pipeline/struct.Pipeline.html) through playback.
    /// The [`Device`](../device/struct.Device.html) available streams are as recorded
    /// to the `file`, and
    /// [`Config::resolve()`](../config/struct.Config.html#method.resolve) considers
    /// only this [`Device`](../device/struct.Device.html) and configuration as
    /// available. This request cannot be used if
    /// [`Config::enable_record_to_file()`](../config/struct.Config.html#method.enable_record_to_file)
    /// is called for the current [`Config`](../config/struct.Config.html), and vise
    /// versa. By default, playback is repeated once the file ends. To control this, see
    /// [`Config::enable_device_from_file_repeat_option()`](../config/struct.Config.html#method.enable_device_from_file_repeat_option).
    ///
    /// # Arguments
    /// * `file` - The playback file of the [`Device`](../device/struct.Device.html).
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_device_from_file(&self, file: &str) -> Result<(), Error> {
        let s = std::ffi::CString::new(file).expect("Failed to create CString");
        Error::call2(rs2::rs2_config_enable_device_from_file, self.0, s.as_ptr())
    }

    /// Select a recorded [`Device`](../device/struct.Device.html) from a `file`, to be
    /// used by the [`Pipeline`](../pipeline/struct.Pipeline.html) through playback.
    /// The [`Device`](../device/struct.Device.html) available streams are as recorded
    /// to the `file`, and
    /// [`Config::resolve()`](../config/struct.Config.html#method.resolve) considers
    /// only this [`Device`](../device/struct.Device.html) and configuration as
    /// available. This request cannot be used if
    /// [`Config::enable_record_to_file()`](../config/struct.Config.html#method.enable_record_to_file)
    /// is called for the current [`Config`](../config/struct.Config.html), and vise
    /// versa.
    ///
    /// # Arguments
    /// * `file` - The playback file of the [`Device`](../device/struct.Device.html).
    /// * `repeat` - If true, when file ends the playback starts again, in an infinite loop. If
    /// false, when `file` ends playback does not start again, and should by stopped manually by
    /// the user.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_device_from_file_repeat_option(
        &self,
        file: &str,
        repeat: bool,
    ) -> Result<(), Error> {
        let s = std::ffi::CString::new(file).expect("Failed to create CString");
        Error::call3(
            rs2::rs2_config_enable_device_from_file_repeat_option,
            self.0,
            s.as_ptr(),
            repeat as i32,
        )
    }

    /// Requires that the resolved [`Device`](../device/struct.Device.html) would be
    /// recorded to file. This request cannot be used if
    /// [`Config::enable_device_from_file()`](../config/struct.Config.html#method.enable_device_from_file)
    /// is called for the current [`Config`](../config/struct.Config.html), and vise
    /// versa.
    ///
    /// # Arguments
    /// * `file` - The desired file for the output record.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_record_to_file(&self, file: &str) -> Result<(), Error> {
        let s = std::ffi::CString::new(file).expect("Failed to create CString");
        Error::call2(rs2::rs2_config_enable_record_to_file, self.0, s.as_ptr())
    }

    /// Disable a [`Device`](../device/struct.Device.html) stream explicitly, to remove
    /// any requests on this stream `type`. The stream can still be enabled due to
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) computer vision module request.
    /// This call removes any filter on the stream configuration.
    ///
    /// # Arguments
    /// * stream - stream type, for which the filters are cleared.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn disable_stream(&self, stream: rs2_stream) -> Result<(), Error> {
        Error::call2(rs2::rs2_config_disable_stream, self.0, stream)
    }

    /// Disable a [`Device`](../device/struct.Device.html) indexed stream explicitly,
    /// to remove any requests on this [`StreamProfile`](../stream_profile/struct.Pipeline.html).
    /// The stream can still be enabled due to [`Pipeline`](../pipeline/struct.Pipeline.html)
    /// computer vision module request. This call removes any filter on the stream configuration.
    ///
    /// # Arguments
    /// * `stream` - Stream type, for which the filters are cleared.
    /// * `index` - Stream index, for which the filters are cleared.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn disable_indexed_stream(&self, stream: rs2_stream, index: i32) -> Result<(), Error> {
        Error::call3(
            rs2::rs2_config_disable_indexed_stream,
            self.0,
            stream,
            index,
        )
    }

    /// Disable all [`Device`](../device/struct.Device.html) streams explicitly, to
    /// remove any requests on the [`StreamsProfile`](struct.StreamsProfile.html)s. The streams can
    /// still be enabled due to [`Pipeline`](../pipeline/struct.Pipeline.html) computer
    /// vision module request. This call removes any filter on the stream configuration.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn disable_all_streams(&self) -> Result<(), Error> {
        Error::call1(rs2::rs2_config_disable_all_streams, self.0)
    }

    /// Resolve the configuration filters, to find a matching
    /// [`Device`](../device/struct.Device.html) and
    /// [`StreamsProfile`](struct.StreamsProfile.html)s. The method resolves the user configuration
    /// filters for the [`Device`](../device/struct.Device.html) and streams, and
    /// combines them with the requirements of the computer vision modules and processing blocks
    /// attached to the pipeline. If there are no conflicts of requests, it looks for an available
    /// [`Device`](../device/struct.Device.html), which can satisfy all requests, and
    /// selects the first matching stream configuration. In the absence of any request, the
    /// [`Config`](../config/struct.Config.html) selects the first available
    /// [`Device`](../device/struct.Device.html) and the first color and `depth` stream
    /// configurations. The
    /// [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html) selection during
    /// [`Pipeline::start()`](../pipeline/struct.Pipeline.html#method.start) follows
    /// the same method. Thus, the selected profile is the same, if no change occurs to the
    /// available [`Device`](../device/struct.Device.html)s occurs. Resolving the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) configuration provides the
    /// application access to the pipeline selected
    /// [`Device`](../device/struct.Device.html) for advanced control. The returned
    /// configuration is not applied to the [`Device`](../device/struct.Device.html),
    /// so the application doesn't own the [`Device`](../device/struct.Device.html)
    /// [`Sensor`](../sensor/struct.Sensor.html)s. However, the application can call
    /// `enable_device()`, to enforce the [`Device`](../device/struct.Device.html)
    /// returned by this method is selected by
    /// [`Pipeline::start()`](../pipeline/struct.Pipeline.html#method.start), and
    /// configure the [`Device`](../device/struct.Device.html) and
    /// [`Sensor`](../sensor/struct.Sensor.html)s options or extensions before
    /// streaming starts.
    ///
    /// # Arguments
    /// * `pipe` - The [`Pipeline`](../pipeline/struct.Pipeline.html) for which the
    /// selected filters are applied.
    ///
    /// # Returns
    /// * `Ok(PipelineProfile)` on success.
    /// * `Err(Error)` on failure.
    pub fn resolve(&self, pipe: &Pipeline) -> Result<PipelineProfile, Error> {
        Error::call2(rs2::rs2_config_resolve, self.0, pipe.handle)
    }

    /// Check if the [`Config`](../config/struct.Config.html) can resolve the
    /// configuration filters, to find a matching [`Device`](../device/struct.Device.html) and
    /// [`StreamProfile`](../stream_profile/struct.Pipeline.html)s. The resolution conditions are
    /// as described in [`Config::resolve()`](../config/struct.Config.html#method.resolve).
    ///
    /// # Arguments
    /// * `pipe` - The [`Pipeline`](../pipeline/struct.Pipeline.html) for which the
    /// selected filters are
    /// applied.
    ///
    /// # Returns
    /// * `Ok(bool)` on success, determining whether the
    /// [`Config`](../config/struct.Config.html) is valid.
    /// * `Err(Error)` on failure.
    pub fn can_resolve(&self, pipe: &Pipeline) -> Result<bool, Error> {
        Error::call2(rs2::rs2_config_can_resolve, self.0, pipe.handle).map(|ret: i32| ret != 0)
    }
}
