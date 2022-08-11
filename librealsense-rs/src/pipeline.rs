// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.

use crate::config::Config;
use crate::context::Context;
use crate::error::Error;
use crate::frame::Frame;
use crate::pipeline_profile::PipelineProfile;

/// Struct representation of [`Pipeline`](../pipeline/struct.Pipeline.html) that wraps around
/// `rs2_pipeline` handle. The [`Pipeline`](../pipeline/struct.Pipeline.html) simplifies the user
/// interaction with the [`Device`](../device/struct.Device.html) and computer vision processing
/// modules. The class abstracts the camera configuration and streaming, and the vision modules
/// triggering and threading. It lets the application focus on the computer vision output of the
/// modules, or the device output data. The [`Pipeline`](../pipeline/struct.Pipeline.html) can
/// manage computer vision modules, which are implemented as a processing blocks. The
/// [`Pipeline`](../pipeline/struct.Pipeline.html) is the consumer of the processing block
/// interface, while the application consumes the computer vision interface.
pub struct Pipeline {
    pub(crate) handle: *mut rs2::rs2_pipeline,
}

/// Safe releasing of the `rs2_pipeline` handle.
impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_pipeline(self.handle) }
    }
}

impl From<*mut rs2::rs2_pipeline> for Pipeline {
    fn from(p: *mut rs2::rs2_pipeline) -> Self {
        Pipeline { handle: p }
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

impl Pipeline {
    /// Create a new [`Pipeline`](../pipeline/struct.Pipeline.html) instance.
    ///
    /// # Arguments
    /// * `ctx` - The [`Context`](../context/struct.Context.html) for which to create a new
    /// [`Pipeline`](../pipeline/struct.Pipeline.html).
    ///
    /// # Returns
    /// * `Ok(Pipeline)` on success.
    /// * `Err(Error)` on failure.
    pub fn create(ctx: &Context) -> Result<Pipeline, Error> {
        Error::call1(rs2::rs2_create_pipeline, ctx.0)
    }

    /// Start the [`Pipeline`](../pipeline/struct.Pipeline.html) streaming with its default
    /// configuration. The pipeline streaming loop captures samples from the
    /// [`Device`](../device/struct.Device.html), and delivers them to the attached computer vision
    /// modules and processing blocks, according to each module requirements and threading model.
    /// During the loop execution, the application can access the camera streams by calling
    /// [`Pipeline::wait_for_frames()`](../pipeline/struct.Pipeline.html#method.wait_for_frames) or
    /// [`Pipeline::poll_for_frames()`](../pipeline/struct.Pipeline.html#method.poll_for_frames).
    /// The streaming loop runs until the [`Pipeline`](../pipeline/struct.Pipeline.html) is
    /// stopped. Starting the [`Pipeline`](../pipeline/struct.Pipeline.html) is possible only when
    /// it is not started. If the [`Pipeline`](../pipeline/struct.Pipeline.html) was started, an
    /// exception is raised.
    ///
    /// # Returns
    /// * `Ok(PipelineProfile)` on success.
    /// * `Err(Error)` on failure.
    pub fn start(&self) -> Result<PipelineProfile, Error> {
        Error::call1(rs2::rs2_pipeline_start, self.handle)
    }

    /// Start the [`Pipeline`](../pipeline/struct.Pipeline.html) streaming according to the
    /// [`Config`](../config/struct.Config.html). The [`Pipeline`](../pipeline/struct.Pipeline.html)
    /// streaming loop captures samples from the [`Device`](../device/struct.Device.html), and
    /// delivers them to the attached computer vision modules and processing blocks, according to
    /// each module requirements and threading model. During the loop execution, the application
    /// can access the camera streams by calling
    /// [`Pipeline::wait_for_frames()`](../pipeline/struct.Pipeline.html#method.wait_for_frames) or
    /// [`Pipeline::poll_for_frames()`](../pipeline/struct.Pipeline.html#method.poll_for_frames).
    /// The streaming loop runs until the [`Pipeline`](../pipeline/struct.Pipeline.html) is
    /// stopped. Starting the [`Pipeline`](../pipeline/struct.Pipeline.html) is possible only when
    /// it is not started. If the [`Pipeline`](../pipeline/struct.Pipeline.html) was started, an
    /// exception is raised. The [`Pipeline`](../pipeline/struct.Pipeline.html) selects and
    /// activates the [`Device`](../device/struct.Device.html) upon start, according to
    /// configuration or a default configuration. The [`Pipeline`](../pipeline/struct.Pipeline.html)
    /// tries to activate the [`Config::resolve()`](../config/struct.Config.html#method.resolve)
    /// result. If the application requests are conflicting with
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) computer vision modules or no matching
    /// [`Device`](../device/struct.Device.html) is available on the platform, the method fails.
    /// Available configurations and [`Device`](../device/struct.Device.html)s may change between
    /// [`Config::resolve()`](../config/struct.Config.html#method.resolve) call and
    /// [`Pipeline::start()`](../pipeline/struct.Pipeline.html#method.start), in case
    /// [`Device`](../device/struct.Device.html)s are connected or disconnected, or another
    /// application acquires ownership of a device.
    ///
    /// # Arguments
    /// * [`Config`](../config/struct.Config.html) - A [`Config`](../config/struct.Config.html)
    /// with requested filters on the [`Pipeline`](../pipeline/struct.Pipeline.html) configuration.
    ///
    /// # Returns
    /// * `Ok(PipelineProfile)` on success.
    /// * `Err(Error)` on failure.
    pub fn start_with_config(&self, rs2_config: &Config) -> Result<PipelineProfile, Error> {
        Error::call2(
            rs2::rs2_pipeline_start_with_config,
            self.handle,
            rs2_config.0,
        )
    }

    /// Stop the [`Pipeline`](../pipeline/struct.Pipeline.html) streaming. The
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) stops delivering samples to the attached
    /// computer vision modules and processing blocks, stops the device streaming and releases the
    /// device resources used by the [`Pipeline`](../pipeline/struct.Pipeline.html). It is the
    /// application's responsibility to release any frame reference it owns. The method takes
    /// effect only after [`Pipeline::start()`](../pipeline/struct.Pipeline.html#method.start) was
    /// called, otherwise an exception is raised.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(Error)` on failure.
    pub fn stop(&self) -> Result<(), Error> {
        Error::call1(rs2::rs2_pipeline_stop, self.handle)
    }

    /// Wait until a new set of [`Frame`](../frame/struct.Frame.html)s becomes available. The
    /// [`Frame`](../frame/struct.Frame.html)s set includes time-synchronized
    /// [`Frame`](../frame/struct.Frame.html)s of each enabled stream in the pipeline. In case of
    /// different frame rates of the streams, the [`Frame`](../frame/struct.Frame.html)s set
    /// include a matching frame of the slow stream, which may have been included in previous
    /// [`Frame`](../frame/struct.Frame.html)s set. The method blocks the calling thread,  and
    /// fetches the latest unread [`Frame`](../frame/struct.Frame.html)s set. Device
    /// [`Frame`](../frame/struct.Frame.html)s, which were produced while the function wasn't
    /// called, are dropped. To avoid frame drops, this method should be called as fast as the
    /// device frame rate. The application can maintain the [`Frame`](../frame/struct.Frame.html)s
    /// handles to defer processing. However, if the application maintains too long history, the
    /// device may lack memory resources to produce new [`Frame`](../frame/struct.Frame.html)s, and
    /// the following call to this method shall fail to retrieve new frames, until resources become
    /// available.
    ///
    /// # Arguments
    /// * `timeout` - Max time in milliseconds to wait until [`Error`](../error/struct.Error.html)
    /// is returned.
    ///
    /// # Returns
    /// * `Ok(Frame)` on success.
    /// * `Err(Error)` on failure.
    pub fn wait_for_frames(&self, timeout_ms: u32) -> Result<Frame, Error> {
        Error::call2(rs2::rs2_pipeline_wait_for_frames, self.handle, timeout_ms)
    }

    /// Check if a new set of [`Frame`](../frame/struct.Frame.html)s is available and retrieve the
    /// latest undelivered set. The [`Frame`](../frame/struct.Frame.html)s set includes
    /// time-synchronized [`Frame`](../frame/struct.Frame.html)s of each enabled stream in the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html). The method returns without blocking the
    /// calling thread, with status of new [`Frame`](../frame/struct.Frame.html)s available or not.
    /// If available, it fetches the latest [`Frame`](../frame/struct.Frame.html)s set. Device
    /// [`Frame`](../frame/struct.Frame.html)s, which were produced while the function wasn't
    /// called, are dropped. To avoid [`Frame`](../frame/struct.Frame.html) drops, this method
    /// should be called as fast as the device [`Frame`](../frame/struct.Frame.html) rate. The
    /// application can maintain the [`Frame`](../frame/struct.Frame.html)s handles to defer
    /// processing. However, if the application maintains too long history, the device may lack
    /// memory resources to produce new [`Frame`](../frame/struct.Frame.html)s, and the following
    /// calls to this method shall return no new [`Frame`](../frame/struct.Frame.html)s, until
    /// resources become available.
    ///
    /// # Returns
    /// * `Ok(Frame)` on success.
    /// * `Err(Error)` on failure.
    pub fn poll_for_frames(&self) -> Result<Option<Frame>, Error> {
        let mut res = Frame(std::ptr::null_mut());
        let ret: i32 = Error::call2(rs2::rs2_pipeline_poll_for_frames, self.handle, &mut res.0)?;
        if ret == 0 {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    }

    /// Return the active [`Device`](../device/struct.Device.html) and streams profiles, used by the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) as
    /// [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html). The
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) streams profiles are selected during
    /// [`Pipeline::start()`](../pipeline/struct.Pipeline.html#method.start). The method returns a
    /// valid result only when the [`Pipeline`](../pipeline/struct.Pipeline.html) is active -
    /// between calls to [`Pipeline::start()`](../pipeline/struct.Pipeline.html#method.start) and
    /// [`Pipeline::stop()`](../pipeline/struct.Pipeline.html#method.stop). After
    /// [`Pipeline::stop()`](../pipeline/struct.Pipeline.html#method.stop) is called, the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) doesn't own the device, thus, the
    /// [`Pipeline`](../pipeline/struct.Pipeline.html) selected device may change in subsequent
    /// activations.
    ///
    /// # Arguments
    /// * `timeout` - Max time in milliseconds to wait until [`Error`](../error/struct.Error.html)
    /// is returned.
    ///
    /// # Returns
    /// * `Ok(PipelineProfile)` on success.
    /// * `Err(Error)` on failure.
    pub fn get_active_profile(&self) -> Result<PipelineProfile, Error> {
        Error::call1(rs2::rs2_pipeline_get_active_profile, self.handle)
    }
}
