use crate::error::Error;
use crate::pipeline::Pipeline;
use crate::pipeline_profile::PipelineProfile;
use rs2;

// Expose `rs2_format` and `rs2_stream` for external use.
pub use rs2::rs2_format;
pub use rs2::rs2_stream;

/// Struct representation of configuration [`Config`](struct.Config.html) that wraps around 
/// `rs2_config` handle. The [`Config`](struct.Config.html) allows, in combination with 
/// [Pipeline](/librealsense2/pipeline/struct.Pipeline.html), to request filters for the streams and
/// [`Device`](/librealsense2/device/struct.Device.html) selection and configuration.
pub struct Config {
    pub(crate) handle: *mut rs2::rs2_config,
}

/// Safe releasing of the `rs2_config` handle.
impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_delete_config(self.handle);
        }
    }
}

impl Config {
    /// Create a [`Config`](struct.Config.html) instance. The [`Config`](struct.Config.html) allows 
    /// [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) users to request filters for the 
    /// [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) streams and [`Device`](struct.Device.html) selection 
    /// and configuration. This is an optional step in [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) creation, 
    /// as the [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) resolves its streaming 
    /// [`Device`](struct.Device.html) internally. [`Config`](struct.Config.html) provides its 
    /// users a way to set the filters and test if there is no conflict with the pipeline 
    /// requirements from the [`Device`](struct.Device.html). It also allows the user to find a 
    /// matching [`Device`](struct.Device.html) for the config filters and the 
    /// [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html), in order to select a [`Device`](struct.Device.html) 
    /// explicitly, and modify its controls before streaming starts.
    ///
    /// # Returns
    /// * `Ok(Config)` on success.
    /// * `Err(Error)` on failure.
    pub fn new() -> Result<Config, Error> {
        let mut error = Error::default();
        let config = Config {
            handle: unsafe { rs2::rs2_create_config(error.inner()) },
        };
        if error.check() {
            Err(error)
        } else {
            Ok(config)
        }
    }

    /// Enable a [`Device`](struct.Device.html) stream explicitly, with selected parameters. The 
    /// method allows the application to request a stream with specific configuration. If no 
    /// stream is explicitly enabled, the [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) configures the 
    /// [`Device`](struct.Device.html) and its streams according to the attached computer vision 
    /// modules and processing blocks requirements, or default configuration for the first 
    /// available [`Device`](struct.Device.html). The application can configure any of the input 
    /// stream parameters according to its requirement, or set to 0 for don't care value. The 
    /// [`Config`](struct.Config.html) accumulates the application calls for enable configuration 
    /// methods, until the configuration is applied. Multiple enable stream calls for the same 
    /// stream with conflicting parameters override each other, and the last call is maintained. 
    /// Upon calling [`Config::resolve()`](struct.Config.html#method.resolve), the 
    /// [`Config`](struct.Config.html) checks for conflicts between the application configuration 
    /// requests and the attached computer vision modules and processing blocks requirements, and 
    /// fails if conflicts are found. Before 
    /// [`Config::resolve()`](struct.Config.html#method.resolve) is called, no conflict check is 
    /// done.
    ///    
    /// # Arguments
    /// * stream - Stream type to be enabled.
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
        let mut error = Error::default();
        unsafe {
            rs2::rs2_config_enable_stream(
                self.handle,
                stream,
                index,
                width,
                height,
                format,
                framerate,
                error.inner(),
            );
        }
        if error.check() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Enable all [`Device`](struct.Device.html) streams explicitly. The conditions and behavior 
    /// of this method are similar to those of 
    /// [`Config::enable_stream()`](struct.Config.html#method.enable_stream). This filter enables 
    /// all raw `Streams` of the selected [`Device`](struct.Device.html). The 
    /// [`Device`](struct.Device.html) is either selected explicitly by the application, or by the 
    /// [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) requirements or default. The list of streams is 
    /// [`Device`](struct.Device.html) dependent.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_all_streams(&self) -> Result<(), Error> {
        let mut error = Error::default();
        unsafe {
            rs2::rs2_config_enable_all_stream(self.handle, error.inner());
        }
        if error.check() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Select a specific [`Device`](struct.Device.html) explicitly by its `serial` number, to be 
    /// used by the [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html).The conditions and behavior of this method 
    /// are similar to those of 
    /// [`Config::enable_stream()`](struct.Config.html#method.enable_stream). This method is 
    /// required if the application needs to set [`Device`](struct.Device.html) or 
    /// [`Sensor`](struct.Sensor.html) settings prior to [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) 
    /// streaming, to enforce the [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) to use the configured 
    /// [`Device`](struct.Device.html).
    ///    
    /// # Arguments
    /// * `serial` - [`Device`](struct.Device.html) serial number, as returned by 
    /// `RS2_CAMERA_INFO_SERIAL_NUMBER`.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_device(&self, serial: &str) -> Result<(), Error> {
        let mut error = Error::default();
        let s = std::ffi::CString::new(serial).expect("Failed to create CString");
        unsafe {
            rs2::rs2_config_enable_device(self.handle, s.as_ptr(), error.inner());
        }
        std::mem::forget(s);
        if error.check() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Select a recorded [`Device`](struct.Device.html) from a `file`, to be used by the 
    /// [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) through playback. The [`Device`](struct.Device.html) 
    /// available streams are as recorded to the `file`, and 
    /// [`Config::resolve()`](struct.Config.html#method.resolve) considers only this 
    /// [`Device`](struct.Device.html) and configuration as available. This request cannot be used 
    /// if [`Config::enable_record_to_file()`](struct.Config.html#method.enable_record_to_file) is 
    /// called for the current [`Config`](struct.Config.html), and vise versa. By default, playback 
    /// is repeated once the file ends. To control this, see 
    /// [`Config::enable_device_from_file_repeat_option()`](struct.Config.html#method.enable_device_from_file_repeat_option).
    ///    
    /// # Arguments
    /// * `file` - The playback file of the [`Device`](struct.Device.html).
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_device_from_file(&self, file: &str) -> Result<(), Error> {
        let mut error = Error::default();
        let s = std::ffi::CString::new(file).expect("Failed to create CString");
        unsafe {
            rs2::rs2_config_enable_device_from_file(self.handle, s.as_ptr(), error.inner());
        }
        std::mem::forget(s);
        if error.check() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Select a recorded [`Device`](struct.Device.html) from a `file`, to be used by the 
    /// [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) through playback. The [`Device`](struct.Device.html) 
    /// available streams are as recorded to the `file`, and 
    /// [`Config::resolve()`](struct.Config.html#method.resolve) considers only this 
    /// [`Device`](struct.Device.html) and configuration as available. This request cannot be used 
    /// if [`Config::enable_record_to_file()`](struct.Config.html#method.enable_record_to_file) is 
    /// called for the current [`Config`](struct.Config.html), and vise versa.
    ///    
    /// # Arguments
    /// * `file` - The playback file of the [`Device`](struct.Device.html).
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
        let mut error = Error::default();
        let s = std::ffi::CString::new(file).expect("Failed to create CString");
        unsafe {
            rs2::rs2_config_enable_device_from_file_repeat_option(
                self.handle,
                s.as_ptr(),
                repeat as i32,
                error.inner(),
            );
        }
        std::mem::forget(s);
        if error.check() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Requires that the resolved [`Device`](struct.Device.html) would be recorded to file. This 
    /// request cannot be used if 
    /// [`Config::enable_device_from_file()`](struct.Config.html#method.enable_device_from_file) is 
    /// called for the current [`Config`](struct.Config.html), and vise versa.
    ///    
    /// # Arguments
    /// * `file` - The desired file for the output record.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn enable_record_to_file(&self, file: &str) -> Result<(), Error> {
        let mut error = Error::default();
        let s = std::ffi::CString::new(file).expect("Failed to create CString");
        unsafe {
            rs2::rs2_config_enable_record_to_file(self.handle, s.as_ptr(), error.inner());
        }
        std::mem::forget(s);
        if error.check() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Disable a [`Device`](struct.Device.html) stream explicitly, to remove any requests on this 
    /// stream `type`. The stream can still be enabled due to [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) 
    /// computer vision module request. This call removes any filter on the stream configuration.
    ///    
    /// # Arguments
    ///	* stream - stream type, for which the filters are cleared.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn disable_stream(&self, stream: rs2_stream) -> Result<(), Error> {
        let mut error = Error::default();
        unsafe {
            rs2::rs2_config_disable_stream(self.handle, stream, error.inner());
        }
        if error.check() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Disable a [`Device`](struct.Device.html) indexed stream explicitly, to remove any requests 
    /// on this [`StreamProfile`](struct.StreamProfile.html). The stream can still be enabled due 
    /// to [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) computer vision module request. This call removes any 
    /// filter on the stream configuration.
    ///    
    /// # Arguments
    ///	* `stream` - Stream type, for which the filters are cleared.
    ///	* `index` - Stream index, for which the filters are cleared.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn disable_indexed_stream(&self, stream: rs2_stream, index: i32) -> Result<(), Error> {
        let mut error = Error::default();
        unsafe {
            rs2::rs2_config_disable_indexed_stream(self.handle, stream, index, error.inner());
        }
        if error.check() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Disable all [`Device`](struct.Device.html) streams explicitly, to remove any requests on 
    /// the [`StreamsProfile`](struct.StreamsProfile.html)s. The streams can still be enabled due 
    /// to [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) computer vision module request. This call removes any 
    /// filter on the stream configuration.
    ///    
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn disable_all_streams(&self) -> Result<(), Error> {
        let mut error = Error::default();
        unsafe {
            rs2::rs2_config_disable_all_streams(self.handle, error.inner());
        }
        if error.check() {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Resolve the configuration filters, to find a matching [`Device`](struct.Device.html) and 
    /// [`StreamsProfile`](struct.StreamsProfile.html)s. The method resolves the user configuration 
    /// filters for the [`Device`](struct.Device.html) and streams, and combines them with the 
    /// requirements of the computer vision modules and processing blocks attached to the pipeline. 
    /// If there are no conflicts of requests, it looks for an available 
    /// [`Device`](struct.Device.html), which can satisfy all requests, and selects the first 
    /// matching stream configuration. In the absence of any request, the 
    /// [`Config`](struct.Config.html) selects the first available [`Device`](struct.Device.html) 
    /// and the first color and `depth` stream configurations. The 
    /// [`PipelineProfile`](struct.PipelineProfile.html) selection during 
    /// [`Pipeline::start()`](/librealsense2/pipeline/struct.Pipeline.html#method.start) follows the same method. Thus, 
    /// the selected profile is the same, if no change occurs to the available 
    /// [`Device`](struct.Device.html)s occurs. Resolving the [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) 
    /// configuration provides the application access to the pipeline selected 
    /// [`Device`](struct.Device.html) for advanced control. The returned configuration is not 
    /// applied to the [`Device`](struct.Device.html), so the application doesn't own the 
    /// [`Device`](struct.Device.html) [`Sensor`](struct.Sensor.html)s. However, the application 
    /// can call `enable_device()`, to enforce the [`Device`](struct.Device.html) returned by this 
    /// method is selected by [`Pipeline::start()`](/librealsense2/pipeline/struct.Pipeline.html#method.start), and 
    /// configure the [`Device`](struct.Device.html) and [`Sensor`](struct.Sensor.html)s options or 
    /// extensions before streaming starts.
    ///    
    /// # Arguments
    ///	* `pipe` - The [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) for which the selected filters are 
    /// applied.
    ///
    /// # Returns
    /// * `Ok(PipelineProfile)` on success.
    /// * `Err(Error)` on failure.
    pub fn resolve(&self, pipe: &Pipeline) -> Result<PipelineProfile, Error> {
        let mut error = Error::default();
        let pipe_profile = PipelineProfile {
            handle: unsafe { rs2::rs2_config_resolve(self.handle, pipe.handle, error.inner()) },
        };

        if error.check() {
            Err(error)
        } else {
            Ok(pipe_profile)
        }
    }

    /// Check if the [`Config`](struct.Config.html) can resolve the configuration filters, to find 
    /// a matching [`Device`](struct.Device.html) and 
    /// [`StreamProfile`](struct.StreamProfile.html)s. The resolution conditions are as described 
    /// in [`Config::resolve()`](struct.Config.html#method.resolve).
    ///    
    /// # Arguments
    ///	* `pipe` - The [`Pipeline`](/librealsense2/pipeline/struct.Pipeline.html) for which the selected filters are 
    /// applied.
    ///
    /// # Returns
    /// * `Ok(bool)` on success, determining whether the [`Config`](struct.Config.html) is valid.
    /// * `Err(Error)` on failure.
    pub fn can_resolve(&self, pipe: &Pipeline) -> Result<bool, Error> {
        let mut error = Error::default();
        let ret = unsafe { rs2::rs2_config_can_resolve(self.handle, pipe.handle, error.inner()) };
        if error.check() {
            Err(error)
        } else {
            if ret == 0 {
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }
}
