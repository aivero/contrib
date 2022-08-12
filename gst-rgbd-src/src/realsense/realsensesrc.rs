// Aivero
// Copyright (C) <2019> Aivero
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Library General Public
// License as published by the Free Software Foundation; either
// version 2 of the License, or (at your option) any later version.
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Library General Public License for more details.
// You should have received a copy of the GNU Library General Public
// License along with this library; if not, write to the
// Free Software Foundation, Inc., 51 Franklin St, Fifth Floor,
// Boston, MA 02110-1301, USA.

use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, RwLock};

use gst::subclass::prelude::*;
use gst_base::prelude::*;
use gst_base::subclass::base_src::CreateSuccess;
use gst_base::subclass::prelude::*;

use camera_meta::Distortion;
use gst::{ErrorMessage, LibraryError};
use gst_depth_meta::{camera_meta, camera_meta::*, rgbd};
use librealsense2::processing::FrameQueue;
use rs2::{frame::Frame, high_level_utils::StreamInfo, processing::ProcessingBlock};

use super::d400_limits::*;
use super::rs_meta::rs_meta_serialization::*;
use super::settings::LogLevel;
use super::settings::*;
use super::streams::*;
use gst_util::taglist::*;
use once_cell::sync::Lazy;

/// The default metric scale for the depth map (1 mm per unit).
const DEFAULT_DEPTH_SCALE: f32 = 0.001;

lazy_static! {
    pub static ref CAT: gst::DebugCategory = gst::DebugCategory::new(
        "realsensesrc",
        gst::DebugColorFlags::empty(),
        Some("Realsense Source"),
    );
}

/// A struct representation of the `realsensesrc` element.
#[derive(Default)]
pub struct RealsenseSrc {
    /// Reconfigurable properties of the element that are protected under RwLock.
    settings: RwLock<Settings>,
    /// Mutex-protected internals of the element containing stream-relevant data.
    internals: Mutex<RealsenseSrcInternals>,
    // Flag signifying that the GstBaseSrc::unlock() method has been called and the create() method should terminate ASAP
    unlock: AtomicBool,
    tags_sent: AtomicBool,
}

/// Internals of the element that are under Mutex.
#[derive(Default)]
struct RealsenseSrcInternals {
    context: Option<rs2::context::Context>,
    pipeline: Option<rs2::pipeline::Pipeline>,
    camera_meta: Option<CameraMeta>,
    /// Contains CameraMeta serialised with Cap'n Proto. Valid only if `attach-camera-meta=true`, otherwise empty.
    camera_meta_serialised: Vec<u8>,
    /// Align processing block that either aligns `depth -> color` or `all streams -> depth`.
    align_processing_block: Option<ProcessingBlock>,
    frame_queue: Option<FrameQueue>,
}

glib::wrapper! {
    pub struct RealsenseSrcObject(ObjectSubclass<RealsenseSrc>)
        @extends gst_base::PushSrc, gst_base::BaseSrc, gst::Element, gst::Object;
}

#[glib::object_subclass]
impl ObjectSubclass for RealsenseSrc {
    const NAME: &'static str = "realsensesrc";
    type Type = RealsenseSrcObject;
    type ParentType = gst_base::PushSrc;
}

impl BaseSrcImpl for RealsenseSrc {
    /// Initialise resources and prepare to produce data.
    /// # Arguments
    /// * `base_src` - Representation of `realsensesrc` element.
    fn start(&self, base_src: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.unlock_stop(base_src)?;
        {
            let settings = self.settings.read().unwrap(); // Specify log severity level for RealSense
            rs2::log::log_to_console(settings.log_level.to_rs2_log_level()).map_err(|e| {
                gst::error_msg!(
                    gst::ResourceError::OpenReadWrite,
                    (&format!("Cannot log librealsense to console: {}", e))
                )
            })?;
        }

        // Make sure that the set properties are viable
        let config = self.configure()?;

        // Configure and start RealSense pipeline
        self.init_realsense_pipeline(base_src.upcast_ref(), config)?;

        gst_info!(CAT, obj: base_src, "Streaming started");

        base_src.set_format(gst::Format::Time);
        base_src.set_property("do-timestamp", &true);

        // Chain up parent implementation
        self.parent_start(base_src)
    }

    /// Close and reset resources.
    /// # Arguments
    /// * `base_src` - Representation of `realsensesrc` element.
    fn stop(&self, base_src: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.unlock(base_src)?;
        // Reset internals
        self.stop_rs_and_reset()?;
        // Chain up parent implementation
        self.parent_stop(base_src)
    }

    /// Called during negotiation if CAPS need fixating. Here we decide on the CAPS based on selected properties.
    /// # Arguments
    /// * `base_src` - Representation of `realsensesrc` element.
    /// * `caps` - CAPS that need to be fixated.
    fn fixate(&self, base_src: &Self::Type, mut caps: gst::Caps) -> gst::Caps {
        caps.truncate();
        {
            let caps = caps.make_mut();

            let s = caps
                .structure_mut(0)
                .expect("Failed to read the realsensesrc CAPS");

            // Create string containing selected streams with priority `depth` > `infra1` > `infra2` > `color`
            // The first stream in this string is contained in the main buffer
            let mut selected_streams = Vec::<String>::new();
            let settings = self.settings.read().unwrap();

            // Iterate over all enabled streams and create corresponding video/rgbd CAPS.
            let streams: Streams = (&settings.streams.enabled_streams).into();
            for (stream_id, stream_descriptor) in streams.iter() {
                selected_streams.push(stream_id.to_string());
                if settings.include_per_frame_metadata {
                    selected_streams.push(format!("{}meta", stream_id));
                }

                // Add the corresponding video format
                s.set(
                    &format!("{}_format", stream_id),
                    &stream_descriptor.video_format.to_string(),
                );

                // Add resolution fields for the stream.
                let (width, height) = match settings.align_to {
                    // Use the configured resolution if aligning is disabled. Infra streams are
                    // not supported by align processing block and always keep their resolution.
                    StreamId::None | StreamId::Infra1 | StreamId::Infra2 => {
                        settings.streams.get_stream_resolution(*stream_id)
                    }
                    // Resolution of the target stream must be used when aligning the stream.
                    // Applies to depth and color streams.
                    _ => settings.streams.get_stream_resolution(settings.align_to),
                };
                s.set(&format!("{}_width", stream_id), &width);
                s.set(&format!("{}_height", stream_id), &height);
            }

            // Add `camerameta` into `streams`, if enabled
            if settings.attach_camera_meta {
                selected_streams.push("camerameta".to_string());
            }
            let selected_streams = selected_streams.iter().map(|s| s.to_send_value());

            // Finally add the streams to the caps
            s.set("streams", &gst::Array::from_values(selected_streams));

            // Fixate the framerate
            s.fixate_field_nearest_fraction("framerate", settings.streams.framerate);
        }

        // Chain up parent implementation
        self.parent_fixate(base_src, caps)
    }

    /// Handle a requested query. Here we explicitely handle Latency query.
    /// # Arguments
    /// * `base_src` - Representation of `realsensesrc` element.
    /// * `query` - The query that was requested.
    fn query(&self, base_src: &Self::Type, query: &mut gst::QueryRef) -> bool {
        use gst::QueryView;
        match query.view_mut() {
            QueryView::Latency(ref mut q) => {
                let settings = self.settings.read().unwrap();
                let framerate: u64 = if let Ok(f) = settings.streams.framerate.try_into() {
                    f
                } else {
                    gst_error!(
                        CAT,
                        obj: base_src,
                        "Could not convert framerate: {} into u64",
                        settings.streams.framerate
                    );
                    return false;
                };
                drop(settings);

                // Setting latency to minimum of 1 frame - 1/framerate
                let latency = if let Some(l) = gst::ClockTime::SECOND.mul_div_floor(1, framerate) {
                    l
                } else {
                    // Return early if we are (most likely) dividing by zero.
                    gst_error!(
                        CAT,
                        obj: base_src,
                        "Could not compute latency, tried to divide 1/{}",
                        framerate
                    );
                    return false;
                };
                gst_debug!(CAT, obj: base_src, "Returning latency {}", latency);
                // Return latency
                q.set(base_src.is_live(), latency, latency);
                true
            }
            _ => BaseSrcImplExt::parent_query(self, base_src, query),
        }
    }

    /// This determines if the source can seek. Seeking during rosbag playback is currently NOT supported.
    /// # Arguments
    /// * `_base_src` - Representation of `realsensesrc` element.
    fn is_seekable(&self, _base_src: &Self::Type) -> bool {
        false
    }

    // Informs the loop in create that we shall not further wait on librealsense to get_frameset
    fn unlock(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.unlock.store(true, Ordering::Relaxed);
        self.parent_unlock(element)
    }

    // Cancels the above notification
    fn unlock_stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        self.unlock.store(false, Ordering::Relaxed);
        self.parent_unlock_stop(element)
    }
}

impl PushSrcImpl for RealsenseSrc {
    /// Create a new buffer that will be pushed downstream.
    /// # Arguments
    /// * `push_src` - Representation of `realsensesrc` element.
    fn create(
        &self,
        push_src: &Self::Type,
        _buffer: Option<&mut gst::BufferRef>,
    ) -> Result<CreateSuccess, gst::FlowError> {
        let duration = {
            let settings = self.settings.read().unwrap();
            gst::ClockTime::from_nseconds(
                std::time::Duration::from_secs_f32(1.0 / settings.streams.framerate as f32)
                    .as_nanos() as u64,
            )
        };

        // Get new frameset from RealSense pipeline
        let mut frameset = self.get_frameset()?;

        // Create the output buffer
        let mut output_buffer = gst::buffer::Buffer::new();

        // Embed the frames in output_buffer
        let settings = &self.settings.read().unwrap();
        let streams: Streams = (&settings.streams.enabled_streams).into();
        {
            let internals = self.internals.lock().unwrap();
            // Align frames, if enabled and configured
            if let (Some(align_processing_block), Some(frame_queue)) =
                (&internals.align_processing_block, &internals.frame_queue)
            {
                align_processing_block
                    .process_frame(frameset)
                    .map_err(|_| gst::FlowError::Error)?;
                frameset = frame_queue
                    .poll_for_frame()
                    .map_err(|_| gst::FlowError::Error)?;
            }
        }

        // Extract individual frames from the frameset
        let number_of_frames = frameset
            .embedded_frames_count()
            .map_err(|_| gst::FlowError::Error)?;
        let frames = (0..number_of_frames)
            .map(|i| frameset.extract_frame(i))
            .collect::<Result<Vec<rs2::frame::Frame>, rs2::error::Error>>()
            .map_err(|_| gst::FlowError::Error)?;

        for (i, (stream_id, stream_descriptor)) in streams.iter().enumerate() {
            // Extract the frame from frames based on its type and id
            let frame = Self::find_frame_with_id(
                &frames,
                stream_descriptor.rs2_stream_descriptor.rs2_stream,
                stream_descriptor.rs2_stream_descriptor.sensor_id,
            )
            .ok_or(gst::FlowError::Error)?;

            // Only the first stream is considered to be 'main'
            let is_stream_main = i == 0;
            self.attach_frame_to_buffer(
                settings,
                &mut output_buffer,
                frame,
                &stream_id.to_string(),
                is_stream_main,
                duration,
            )
            .map_err(|_| gst::FlowError::Error)?;
        }

        // Attach Cap'n Proto serialised `CameraMeta` if enabled
        if settings.attach_camera_meta {
            // An explicit clone of the serialised buffer is used so that CameraMeta does not need to be serialised every time.
            let camera_meta = self
                .internals
                .lock()
                .unwrap()
                .camera_meta_serialised
                .clone();
            self.attach_camera_meta(&mut output_buffer, camera_meta, duration)
                .map_err(|_| gst::FlowError::Error)?;
        }

        if !self.tags_sent.swap(true, Ordering::Acquire) {
            if let Some(camera_meta) = self.internals.lock().unwrap().camera_meta.clone() {
                let src_pad = push_src.static_pad("src").unwrap();
                Self::send_pipeline_tags(&camera_meta, &src_pad)
                    .map_err(|_| gst::FlowError::Error)?;
            }
        }
        Ok(CreateSuccess::NewBuffer(output_buffer))
    }
}

impl RealsenseSrc {
    /// Configure the RealSense pipeline, while making sure the settings are valid.
    /// # Returns
    /// * `Ok(rs2::config::Config)` if realsenesrc could be configured to use serial or rosbag
    /// * `Err(LibraryError)` if
    ///   * Neither serial, nor rosbag_location are specified
    ///   * BOTH serial and rosbag_location are specified
    ///   * No streams are enabled
    fn configure(&self) -> Result<rs2::config::Config, gst::ErrorMessage> {
        // Create new RealSense config
        let config = rs2::config::Config::create()
            .map_err(|e| gst::error_msg!(gst::LibraryError::Settings, ["{}", e]))?;

        let settings = self.settings.read().unwrap();

        // At least one stream must be enabled
        if !settings.streams.enabled_streams.any() {
            return Err(gst::error_msg!(
                gst::LibraryError::Failed,
                ["No stream is enabled. At least one stream must be enabled"]
            ));
        }
        // Either `serial` or `rosbag-location` must be specified
        match (&settings.serial, &settings.rosbag_location) {
            // Stream from a physical camera
            (Some(serial), None) => {
                // Enable the selected streams
                Self::enable_streams(&config, &settings)
                    .map_err(|e| gst::error_msg!(gst::LibraryError::Settings, ["{}", e]))?;

                // Enable device with the given serial number and device configuration
                config.enable_device(serial)
                    .map_err(|e| gst::error_msg!(LibraryError::Settings, ["{}", e]))?;
                Ok(config)
            }
            // Stream from rosbag
            (None, Some(rosbag)) => {
                config.enable_device_from_file_repeat_option(rosbag, settings.loop_rosbag)
                    .map_err(|e| gst::error_msg!(gst::LibraryError::Settings, ["{}", e]))?;
                Ok(config)
            }
            // Make sure that exactly one stream source is selected
            (None, None) => {
                Err(
                    gst::error_msg!(
                        gst::StreamError::Failed,
                        ["Neither the `serial` or `rosbag-location` properties are defined. At least one of these must be defined!"]
                    )
                )
            }
            (Some(serial), Some(rosbag_location)) => {
                Err(
                    gst::error_msg!(
                        gst::StreamError::Failed,
                        ["Both `serial`: {:?} and `rosbag-location`: {:?} are defined. Only one of these can be defined!", serial, rosbag_location]
                    )
                )
            }
        }
    }

    /// Enable all the streams that has their associated property set to `true`.
    /// # Arguments
    /// * `config` - The realsense configuration, which may be used to enable streams.
    /// * `settings` - The settings for the realsensesrc, which in this case specifies which streams to enable.
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(gst::ErrorMessage)` if any of the streams cannot be enabled.
    fn enable_streams(
        config: &rs2::config::Config,
        settings: &Settings,
    ) -> Result<(), gst::ErrorMessage> {
        // Iterate over all user-enabled streams and enable them in librealsense config
        let streams: Streams = (&settings.streams.enabled_streams).into();
        for (stream_id, stream_descriptor) in streams.iter() {
            let (width, height) = settings.streams.get_stream_resolution(*stream_id);
            config
                .enable_stream(
                    stream_descriptor.rs2_stream_descriptor.rs2_stream,
                    stream_descriptor.rs2_stream_descriptor.sensor_id,
                    width,
                    height,
                    stream_descriptor.rs2_stream_descriptor.rs2_format,
                    settings.streams.framerate,
                )
                .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;
        }
        Ok(())
    }

    /// Prepare a librealsense pipeline, which can read frames from a RealSense camera, using the
    /// given `config` and `settings`. If the preparation succeeds, the pipeline is started.
    /// # Arguments
    /// * `base_src` - Representation of `realsensesrc` element.
    /// * `config` - The librealsense configuration to use for the camera.
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(LibraryError)` if resolving `config` fails.
    fn init_realsense_pipeline(
        &self,
        base_src: &gst_base::BaseSrc,
        config: rs2::config::Config,
    ) -> Result<(), ErrorMessage> {
        // Get realsense context
        let context = rs2::context::Context::create()
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;

        let mut settings = self.settings.write().unwrap();
        if let (Some(serial), Some(config)) = (&settings.serial, &settings.config) {
            // Load JSON if `config` is defined
            let device = Self::find_device(&context, serial)?;
            Self::load_json(&device, config)?;
        }

        // Crate new RealSense pipeline
        let pipeline = rs2::pipeline::Pipeline::create(&context)
            .map_err(|e| gst::error_msg!(gst::CoreError::Failed, ["{}", e]))?;

        // Make sure that the config can be resolved
        // Note that these variants are obtained directly from C librealsense API as strings,
        // here we just expand the error messages to make the user informed in a better way.
        config.resolve(&pipeline).map_err(|e| {
            let err = e.get_message();
            match err.as_str() {
                "No device connected" => {
                    gst::error_msg!(
                        gst::StreamError::Failed,
                        ["Device with serial '{}' is not connected", settings.serial.as_ref().unwrap()]
                    )
                }
                "Couldn't resolve requests" => {
                    gst::error_msg!(
                        gst::StreamError::Failed,
                        ["The selected RealSense configuration is NOT supported by your device:{}", settings.streams]
                    )
                }
                _ => gst::error_msg!(
                    gst::StreamError::Failed,
                        ["{}", err]
                    )
            }
        })?;

        // Start the RealSense pipeline
        let pipeline_profile = pipeline
            .start_with_config(&config)
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;

        // If playing from a rosbag recording, check whether the correct properties were selected
        // and update them
        if settings.rosbag_location.is_some() {
            self.configure_rosbag_settings(&mut settings, &pipeline_profile)?;
        }

        // Extract and print camera meta
        let camera_meta =
            Self::get_camera_meta(&settings.streams.enabled_streams, &pipeline_profile)?;
        gst_info!(
            CAT,
            obj: base_src,
            "RealSense stream source has the following calibration:\n{:?}",
            camera_meta
        );

        let mut internals = self.internals.lock().unwrap();
        internals.context = Some(context);
        internals.pipeline = Some(pipeline);

        // Setup camera meta for transport, if enabled
        internals.camera_meta = Some(camera_meta.clone());
        if settings.attach_camera_meta {
            // Serialise the CameraMeta
            internals.camera_meta_serialised = camera_meta.serialise().map_err(|e| {
                gst::error_msg!(
                    gst::LibraryError::Settings,
                    ["Cannot serialise camera meta{}", e]
                )
            })?
        }

        // Create align processing block if enabled
        if !matches!(
            settings.align_to,
            StreamId::Infra1 | StreamId::Infra2 | StreamId::None
        ) {
            if !settings.streams.enabled_streams.depth {
                gst_warning!(
                    CAT,
                    "Can not perform alignment if depth stream is not enabled"
                );
                return Ok(());
            }
            if !settings.streams.enabled_streams.color && settings.align_to == StreamId::Color {
                gst_warning!(
                    CAT,
                    "Can not align depth to color if color stream is not enabled"
                );
                return Ok(());
            }
            // librealsense segfaults if we try to align zero streams to depth.
            // We prefer it warns and then operates as if no alignment was requested.
            let enabled_streams: Streams = (&settings.streams.enabled_streams).into();
            if enabled_streams.len() > 1 {
                internals.align_processing_block = Some(
                    ProcessingBlock::create_align(
                        Into::<RsStreamDescriptor>::into(settings.align_to).rs2_stream,
                    )
                    .map_err(|e| gst::error_msg!(gst::LibraryError::Settings, ["{}", e]))?,
                );
                internals.frame_queue = Some(
                    FrameQueue::create(1)
                        .map_err(|e| gst::error_msg!(gst::LibraryError::Settings, ["{}", e]))?,
                );
                internals
                    .align_processing_block
                    .as_ref()
                    .unwrap()
                    .start(internals.frame_queue.as_ref().unwrap())
                    .map_err(|e| gst::error_msg!(gst::LibraryError::Settings, ["{}", e]))?;
            } else {
                gst_info!(
                    CAT,
                    "Alignment requested but only depth is enabled. Nothing to do."
                );
            }
        }

        Ok(())
    }

    fn find_device(
        context: &rs2::context::Context,
        serial: &str,
    ) -> Result<rs2::device::Device, gst::ErrorMessage> {
        let devices = context.query_devices().map_err(|e| {
            gst::error_msg!(gst::StreamError::Failed, ["Unable to query devices: {}", e])
        })?;

        // Make sure a device with the selected serial is connected
        // Find the device with the given serial, ignoring all errors
        let number_of_devices = devices.count().map_err(|e| {
            gst::error_msg!(
                gst::StreamError::Failed,
                ["Unable to get device count {}", e]
            )
        })?;
        (0..number_of_devices)
            .filter_map(|i| devices.create_device(i).ok())
            .find(
                |d| match d.get_info(rs2::rs2_camera_info::RS2_CAMERA_INFO_SERIAL_NUMBER) {
                    Ok(device_serial) => *serial == device_serial,
                    _ => false,
                },
            )
            .ok_or_else(|| {
                gst::error_msg!(
                    gst::StreamError::Failed,
                    ["Could not find device for serial {}", serial]
                )
            })
    }

    /// Send the metadata as tags on the pipeline in order to be read by downstream elements.
    /// # Arguments
    /// * `camera_metadata` - Metadate to send.
    /// * `src_pad` - The src pad.
    fn send_pipeline_tags(camera_meta: &CameraMeta, pad: &gst::Pad) -> Result<(), ErrorMessage> {
        let serialised_meta = serde_json::to_string(camera_meta)
            .map_err(|e| gst::error_msg!(LibraryError::Failed, ["{}", e]))?;
        gst_info!(CAT, "Sending metadata as tags:{:#?}", serialised_meta);

        let tags = gst::TagList::new_single::<CameraMetaTag>(&serialised_meta.as_str());
        // Tags need to be send (a) after the segment event AND (b) before the first buffer is being pushed.
        // Storing the sticky event on the pad is the least ugly way to ensure that happens. (slomo suggestion)
        // fixme: Change to a yet to be created `gst_base_src_set_tags` API
        pad.store_sticky_event(&gst::event::Tag::new(tags))
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;
        Ok(())
    }

    /// Configure the device with the JSON file specified on the given `json_location`.
    /// # Arguments
    /// * `device` - Device to configure.
    /// * `json_location` - The absolute path to the file containing the JSON configuration.
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(RealsenseError)` if
    ///     * Invalid `serial` is passed
    ///     * Json file cannot be read
    ///     * Json config is invalid
    fn load_json(device: &rs2::device::Device, config: &str) -> Result<(), ErrorMessage> {
        if !device
            .is_enabled()
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?
        {
            device
                .set_advanced_mode(true)
                .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;
        }

        device
            .load_json(config)
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;
        Ok(())
    }

    /// Get a new set of synchronised frames from RealSense pipeline.
    /// # Returns
    /// * `Ok(Vec<rs2::frame::Frame>)` on success.
    /// * `Err(gst::FlowError::Eos)` if no new frames are available.
    /// # Panics
    /// * If RealSense pipeline has not yet started
    fn get_frameset(&self) -> Result<Frame, gst::FlowError> {
        let internals = self.internals.lock().unwrap();

        // Get RealSense pipeline
        let pipeline = internals.pipeline.as_ref().unwrap();
        let mut waited = gst::ClockTime::from_nseconds(0);

        // Wait for frameset, i.e. rs2::Frame with multiple Frames attached to it
        // Wait 10ms and check (a) if unlock() has been called and (b) we get a frameset or (c) we exceeded the max timeout
        // Allows for terminating the pipeline quicker that the `wait_for_frames_timeout`
        loop {
            if self.unlock.load(Ordering::Relaxed) {
                return Err(gst::FlowError::Flushing);
            }
            match pipeline.wait_for_frames(10) {
                Ok(frameset) => return Ok(frameset),
                Err(err) => {
                    gst::gst_info!(CAT, "wait_for_frameset returned {}", err);
                    waited += gst::ClockTime::from_mseconds(10);
                }
            }
            let timeout = self.settings.read().unwrap().wait_for_frames_timeout;
            let timeout = gst::ClockTime::from_mseconds(timeout.into());
            if waited > timeout {
                gst::gst_error!(CAT, "Timed out while getting frameset");
                return Err(gst::FlowError::CustomError);
            }
        }
    }

    /// Attempt to read the metadata from the given frame and serialize it using CapnProto. If this
    /// function returns `None`, it prints a warning to console that explains the issue.
    /// # Arguments
    /// * `frame` - The frame to read and serialize metadata for.
    /// # Returns
    /// * `Ok(Vec<u8>)` on success.
    /// * `Err(gst::FlowError::Eos)` if metadata cannot be acquired.
    fn get_frame_meta(&self, frame: &Frame) -> Result<Vec<u8>, ErrorMessage> {
        let frame_meta = frame
            .get_metadata()
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;
        capnp_serialize(frame_meta).map_err(|e| {
            gst::error_msg!(
                gst::StreamError::Failed,
                ["Failed to serialize metadata from RealSense camera: {}", e]
            )
        })
    }

    /// Attempt to add `frame_meta` as a gst meta buffer onto `buffer`. This function simply ignores
    /// cases there `frame_meta` is `None`.
    /// # Arguments
    /// * `buffer` - The gst::Buffer to which the metadata should be added.
    /// * `frame_meta` - A byte vector containing the serialized metadata.
    /// * `tag` - The tag of the stream.
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(RealsenseError)` if `frame_meta` cannot be attached to the `buffer`.
    fn add_per_frame_metadata(
        &self,
        buffer: &mut gst::BufferRef,
        frame_meta: Vec<u8>,
        tag: &str,
        duration: gst::ClockTime,
    ) -> Result<(), ErrorMessage> {
        // If we were able to read some metadata add it to the buffer
        let mut frame_meta_buffer = gst::buffer::Buffer::from_slice(frame_meta);
        buffer.set_duration(duration);

        // Attach the meta buffer and tag it adequately
        rgbd::attach_aux_buffer_and_tag(buffer, &mut frame_meta_buffer, &format!("{}meta", tag))?;

        Ok(())
    }

    /// Extract a frame from the RealSense camera, outputting it in `output_buffer` on the given
    /// `push_src`. This function outputs the frame as main buffer if `is_buffer_main` is *true* or
    ///  as a meta buffer if `is_buffer_main` is *false*.
    /// # Arguments
    /// * `push_src` - The element that represents the `realsensesrc`.
    /// * `settings` - The settings for the `realsensesrc`.
    /// * `output_buffer` - The buffer which the frames should be extracted into.
    /// * `frame` - The frame to attach to the buffer
    /// * `tag` - The tag to give to the buffer. This may be used to identify the type of the stream later downstream.
    /// * `is_buffer_main` - A flag that determine whether the currently proccessed buffer is main or auxiliary.
    #[allow(clippy::too_many_arguments)]
    fn attach_frame_to_buffer(
        &self,
        settings: &Settings,
        output_buffer: &mut gst::Buffer,
        frame: &Frame,
        tag: &str,
        is_buffer_main: bool,
        duration: gst::ClockTime,
    ) -> Result<(), ErrorMessage> {
        // Extract the frame data into a new buffer
        let frame_data = frame
            .get_data()
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e.get_message()]))?;
        let mut buffer = gst::buffer::Buffer::with_size(frame_data.len()).unwrap();

        // Newly allocated buffer is mutable, no need for error handling
        let buffer_mut_ref = buffer.get_mut().unwrap();
        buffer_mut_ref.copy_from_slice(0, frame_data).unwrap();
        buffer_mut_ref.set_duration(duration);

        // Where the buffer is placed depends whether this is the first stream that is enabled
        if is_buffer_main {
            // Fill the main buffer and tag it adequately
            rgbd::fill_main_buffer_and_tag(output_buffer, buffer, tag)?;
        } else {
            // Attach the auxiliary buffer and tag it adequately
            rgbd::attach_aux_buffer_and_tag(
                output_buffer.get_mut().ok_or_else(|| {
                    gst::error_msg!(
                        gst::StreamError::Failed,
                        [
                            "Cannot get mutable reference to the main buffer while attaching {} \
                             stream",
                            tag
                        ]
                    )
                })?,
                &mut buffer,
                tag,
            )?;
        }

        // Check if we should attach RealSense per-frame meta and do that if so
        if settings.include_per_frame_metadata {
            // Attempt to read the RealSense per-frame metadata, otherwise set frame_meta to None
            let md = self.get_frame_meta(frame)?;
            self.add_per_frame_metadata(
                output_buffer.get_mut().ok_or_else(|| {
                    gst::error_msg!(
                        gst::StreamError::Failed,
                        [
                            "Cannot get mutable reference to the main buffer while attaching \
                             {}meta",
                            tag
                        ]
                    )
                })?,
                md,
                tag,
                duration,
            )?;
        }

        Ok(())
    }

    /// Check if the selected settings match enabled streams and their properties in rosbag file.
    /// If an enabled stream is not contained within a rosbag recording, this function returns
    /// error. If different stream resolutions or a different framerate were selected, this
    /// function updates them appropriately based on the information contained within the rosbag
    /// recording.
    /// # Arguments
    /// * `settings` - The configured properties of the element
    /// * `pipeline_profile` - The profile of the current realsense pipeline.
    /// # Returns
    /// * `Ok()` if all enabled streams are available. Settings for these streams might get updated.
    /// * `Err(RealsenseError)` if an enabled stream is not available in rosbag recording.
    fn configure_rosbag_settings(
        &self,
        settings: &mut Settings,
        pipeline_profile: &rs2::pipeline_profile::PipelineProfile,
    ) -> Result<(), ErrorMessage> {
        let stream_settings = &mut settings.streams;
        // Get information about the streams in the rosbag recording.
        let stream_infos = rs2::high_level_utils::get_info_all_streams(pipeline_profile)
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;

        // Create a struct of enabled streams that is later used to check whether some of the
        // enabled streams if not contained in the rosbag recording.
        let mut rosbag_enabled_streams = EnabledStreams {
            depth: false,
            infra1: false,
            infra2: false,
            color: false,
        };

        // Iterate over all streams contained in the rosbag recording
        for stream_info in &stream_infos {
            self.update_settings_from_rosbag(
                stream_info,
                stream_settings,
                &mut rosbag_enabled_streams,
            );
        }

        // Return error if at least of the enabled streams is not contained within the
        // rosbag recording.
        self.check_if_streams_are_available(
            &stream_settings.enabled_streams,
            &rosbag_enabled_streams,
        )?;

        // Set real-time playback based on property
        let playback = pipeline_profile
            .get_device()
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;
        playback
            .set_real_time(settings.real_time_rosbag_playback)
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;

        Ok(())
    }

    /// Update stream resolutions or framerate if there is a conflict between settings and the
    /// rosbag recording.
    /// # Arguments
    /// * `stream_info` - The information of a stream obtained from rosbag recording.
    /// * `stream_settings` - The settings selected for the streams.
    /// * `rosbag_enabled_streams` - A list of what streams are enabled in the rosbag.
    /// # Panics
    /// * If `stream_info` with invalid index is passed in.
    fn update_settings_from_rosbag(
        &self,
        stream_info: &StreamInfo,
        stream_settings: &mut StreamsSettings,
        rosbag_enabled_streams: &mut EnabledStreams,
    ) {
        // Determine what stream to consider
        match stream_info.data.stream {
            // Consider `depth` stream
            rs2::rs2_stream::RS2_STREAM_DEPTH => {
                rosbag_enabled_streams.depth = true;

                // Uptade `depth` stream, if applicable
                self.update_stream(
                    "depth",
                    stream_info,
                    stream_settings.enabled_streams.depth,
                    &mut stream_settings.depth_resolution,
                    &mut stream_settings.framerate,
                );
            }

            // Consider one of `infraX` streams
            rs2::rs2_stream::RS2_STREAM_INFRARED => {
                match stream_info.data.index {
                    // Consider `infra1`
                    1 => {
                        rosbag_enabled_streams.infra1 = true;

                        // Uptade `infra1` stream, if applicable
                        self.update_stream(
                            "infra1",
                            stream_info,
                            stream_settings.enabled_streams.infra1,
                            &mut stream_settings.depth_resolution,
                            &mut stream_settings.framerate,
                        );
                    }
                    // Consider `infra2`
                    2 => {
                        rosbag_enabled_streams.infra2 = true;

                        // Uptade `infra2` stream, if applicable
                        self.update_stream(
                            "infra2",
                            stream_info,
                            stream_settings.enabled_streams.infra2,
                            &mut stream_settings.depth_resolution,
                            &mut stream_settings.framerate,
                        );
                    }
                    // There are only 2 sensors in a binocular stereo setup
                    _ => unimplemented!(),
                }
            }

            // Consider `color`
            rs2::rs2_stream::RS2_STREAM_COLOR => {
                rosbag_enabled_streams.color = true;

                // Uptade `color` stream, if applicable
                self.update_stream(
                    "color",
                    stream_info,
                    stream_settings.enabled_streams.color,
                    &mut stream_settings.color_resolution,
                    &mut stream_settings.framerate,
                );
            }
            // No other streams are expected
            _ => unimplemented!(),
        }
    }

    /// Update stream resolutions or framerate if there is a conflict between settings and the
    /// rosbag recording.
    ///
    /// # Arguments
    /// * `stream_id` - The identifier of the stream.
    /// * `stream_info` - The informaton about the stream from rosbag recording.
    /// * `stream_settings_enabled` - Determines whether the stream enabled.
    /// * `stream_settings_resolution` - The selected resolution of the stream.
    /// * `stream_settings_framerate` - The selected framerate of the stream.
    fn update_stream(
        &self,
        stream_id: &str,
        stream_info: &StreamInfo,
        stream_settings_enabled: bool,
        stream_settings_resolution: &mut StreamResolution,
        stream_settings_framerate: &mut i32,
    ) {
        // There is no need to update if the stream is not even enabled.
        if stream_settings_enabled {
            // Update the stream resolution, if applicable
            self.update_resolution_based_on_rosbag(
                stream_id,
                stream_settings_resolution,
                &stream_info.resolution,
            );

            // Update the stream framerate, if applicable
            self.update_framerate_based_on_rosbag(
                stream_id,
                stream_settings_framerate,
                stream_info.data.framerate,
            );
        } else {
            // Notify STDOUT that there is a stream that was not enabled
            gst_info!(
                CAT,
                "There is a `{}` stream contained within the rosbag recording that was not enabled.", stream_id
            );
        }
    }

    /// Update settings for the resolution of a stream if a conflict with rosbag is detected.
    ///
    /// # Arguments
    /// * `stream_id` - The identifier of the stream.
    /// * `settings_resolution` - The resolution selected in the settings stream.
    /// * `rosbag_resolution` - The actual resolution of the rosbag stream.
    fn update_resolution_based_on_rosbag(
        &self,
        stream_id: &str,
        settings_resolution: &mut StreamResolution,
        rosbag_resolution: &StreamResolution,
    ) {
        if settings_resolution != rosbag_resolution {
            gst_warning!(
                CAT,
                "The selected resolution of {}x{} px for the `{}` stream differs from the resolution in the rosbag recording. Setting the stream's resolution to {}x{} px.",
                settings_resolution.width,
                settings_resolution.height,
                stream_id,
                rosbag_resolution.width,
                rosbag_resolution.height
            );
            *settings_resolution = rosbag_resolution.clone();
        }
    }

    /// Update settings for the framerate of a stream if a conflict with rosbag is detected.
    ///
    /// # Arguments
    /// * `stream_id` - The identifier of the stream.
    /// * `settings_framerate` - The framerate selected in the settings stream.
    /// * `rosbag_framerate` - The actual framerate of the rosbag stream.
    fn update_framerate_based_on_rosbag(
        &self,
        stream_id: &str,
        settings_framerate: &mut i32,
        rosbag_framerate: i32,
    ) {
        if settings_framerate != &rosbag_framerate {
            gst_warning!(
                CAT,
                "The selected framerate of {} fps for the `{}` stream differs from the framerate in the rosbag recording. Setting the stream's framerate to {} fps.",
                settings_framerate,
                stream_id,
                rosbag_framerate,
            );
            *settings_framerate = rosbag_framerate;
        }
    }

    /// Check if all the enabled streams are available.
    ///
    /// # Arguments
    /// * `enabled_streams` - The selected streams.
    /// * `available_streams` - The actual available streams.
    ///
    /// # Returns
    /// * `Ok()` if all enabled streams are available.
    /// * `Err(RealsenseError)` if an enabled stream is not available.
    fn check_if_streams_are_available(
        &self,
        enabled_streams: &EnabledStreams,
        available_streams: &EnabledStreams,
    ) -> Result<(), ErrorMessage> {
        let conflicting_streams = enabled_streams.get_conflicts(available_streams);

        if !conflicting_streams.is_empty() {
            return Err(gst::error_msg!(
                gst::StreamError::Failed,
                [
                    "The following stream(s) `{:?}` are not available in the rosbag recording.",
                    conflicting_streams,
                ]
            ));
        }

        Ok(())
    }

    /// Sets up the serialised CameraMeta from Realsense PipelineProfile.
    ///
    /// # Arguments
    /// * `desired_streams` - List of desired streams.
    /// * `pipeline_profile` - RealSense PipelineProfile.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(RealsenseError)` on failure.
    fn get_camera_meta(
        desired_streams: &EnabledStreams,
        pipeline_profile: &rs2::pipeline_profile::PipelineProfile,
    ) -> Result<CameraMeta, ErrorMessage> {
        // Get the sensors and active stream profiles from the pipeline profile
        let sensors = pipeline_profile
            .get_device()
            .unwrap()
            .query_sensors()
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;
        let stream_profiles = pipeline_profile
            .get_streams()
            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;

        // Create intrinsics and insert the appropriate streams
        let intrinsics = Self::extract_intrinsics(desired_streams, &stream_profiles)?;

        // Create extrinsics and insert the appropriate transformations
        let extrinsics = Self::extract_extrinsics(desired_streams, &stream_profiles)?;

        // Create camera meta from the intrinsics, extrinsics and depth scale
        Ok(CameraMeta::new(
            intrinsics,
            extrinsics,
            Self::get_depth_scale(sensors),
        ))
    }

    /// Extract Intrinsics from the active RealSense stream profiles, while taking into account what streams are enabled.
    ///
    /// # Arguments
    /// * `desired_streams` - Desired streams.
    /// * `stream_profiles` - Active stream profiles.
    ///
    /// # Returns
    /// * `HashMap<String, camera_meta::Intrinsics>` containing Intrinsics corresponding to a stream.
    fn extract_intrinsics(
        desired_streams: &EnabledStreams,
        stream_profiles: &rs2::stream_profile::StreamProfileList,
    ) -> Result<HashMap<String, camera_meta::Intrinsics>, ErrorMessage> {
        let mut intrinsics: HashMap<String, camera_meta::Intrinsics> = HashMap::new();

        // Iterate over all stream profile, extract intrinsics and assign them to the appropriate stream
        for i in 0..stream_profiles.count().unwrap() {
            let stream_profile = stream_profiles.get(i).unwrap();
            let stream_data = stream_profile
                .get_data()
                .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;
            let stream_id: StreamId =
                RsStreamDescriptor::new(stream_data.stream, stream_data.format, stream_data.index)
                    .into();

            // Make sure that the stream is enabled for streaming
            if Self::is_stream_enabled(stream_id, desired_streams) {
                intrinsics.insert(
                    stream_id.to_string(),
                    Self::rs2_intrinsics_to_camera_meta_intrinsics(
                        stream_profile
                            .get_intrinsics()
                            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?,
                    ),
                );
            }
        }

        Ok(intrinsics)
    }

    /// Convert Realsense Extrinsics into CameraMeta Intrinsics.
    ///
    /// # Arguments
    /// * `rs2_intrinsics` - RealSense intrinsics to convert.
    ///
    /// # Returns
    /// * `camera_meta::Intrinsics` containing the converted intrinsics.
    fn rs2_intrinsics_to_camera_meta_intrinsics(
        rs2_intrinsics: rs2::intrinsics::Intrinsics,
    ) -> camera_meta::Intrinsics {
        use rs2::intrinsics::Distortion as rs2_dis;

        let distortion = match rs2_intrinsics.model {
            rs2_dis::RS2_DISTORTION_NONE => Distortion::None,
            rs2_dis::RS2_DISTORTION_MODIFIED_BROWN_CONRADY => Distortion::RsModifiedBrownConrady(
                camera_meta::RsCoefficients::from(rs2_intrinsics.coeffs),
            ),
            rs2_dis::RS2_DISTORTION_INVERSE_BROWN_CONRADY => Distortion::RsInverseBrownConrady(
                camera_meta::RsCoefficients::from(rs2_intrinsics.coeffs),
            ),
            rs2_dis::RS2_DISTORTION_FTHETA => {
                Distortion::RsFTheta(camera_meta::RsCoefficients::from(rs2_intrinsics.coeffs))
            }
            rs2_dis::RS2_DISTORTION_BROWN_CONRADY => {
                Distortion::RsBrownConrady(camera_meta::RsCoefficients::from(rs2_intrinsics.coeffs))
            }
            rs2_dis::RS2_DISTORTION_KANNALA_BRANDT4 => Distortion::RsKannalaBrandt4(
                camera_meta::RsCoefficients::from(rs2_intrinsics.coeffs),
            ),
            rs2_dis::RS2_DISTORTION_COUNT => {
                unreachable!("RS2_DISTORTION_COUNT is not a valid distotion model")
            }
        };

        camera_meta::Intrinsics {
            fx: rs2_intrinsics.fx,
            fy: rs2_intrinsics.fy,
            cx: rs2_intrinsics.ppx,
            cy: rs2_intrinsics.ppy,
            distortion,
        }
    }

    /// Extract extrinsics from the active RealSense stream profiles, while taking into account what streams are enabled.
    ///
    /// # Arguments
    /// * `desired_streams` - Desired streams.
    /// * `stream_profiles` - Active stream profiles.
    ///
    /// # Returns
    /// * `HashMap<(String, String), camera_meta::Transformation>` containing Transformation
    /// in a hashmap of <(from, to), Transformation>.
    fn extract_extrinsics(
        desired_streams: &EnabledStreams,
        stream_profiles: &rs2::stream_profile::StreamProfileList,
    ) -> Result<HashMap<(String, String), camera_meta::Transformation>, ErrorMessage> {
        // Determine the main stream from which all transformations are taken
        let main_stream_id = Self::determine_main_stream(desired_streams);
        let main_stream_rs_descriptor: RsStreamDescriptor = main_stream_id.into();

        // Get the stream profile for the main stream
        let main_stream_profile = (0..stream_profiles.count().unwrap())
            .filter_map(|i| stream_profiles.get(i).ok())
            .find(|stream_profile| match stream_profile.get_data() {
                Ok(stream) => {
                    stream.stream == main_stream_rs_descriptor.rs2_stream
                        && if main_stream_rs_descriptor.sensor_id == -1 {
                            true
                        } else {
                            stream.index == main_stream_rs_descriptor.sensor_id
                        }
                }
                _ => false,
            })
            .expect("There is no stream profile for the primary enabled stream");

        // Iterate over all stream profiles and find extrinsics to the other enabled streams
        let mut extrinsics: HashMap<(String, String), camera_meta::Transformation> = HashMap::new();
        for i in 0..stream_profiles.count().unwrap() {
            let stream_profile = stream_profiles.get(i).unwrap();
            let stream_data = stream_profile
                .get_data()
                .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?;
            let stream_id: StreamId =
                RsStreamDescriptor::new(stream_data.stream, stream_data.format, stream_data.index)
                    .into();

            if stream_id == main_stream_id {
                // Skip the main buffer
                continue;
            }

            // Make sure that the stream is enabled for streaming
            if Self::is_stream_enabled(stream_id, desired_streams) {
                extrinsics.insert(
                    (main_stream_id.to_string(), stream_id.to_string()),
                    Self::rs2_extrinsics_to_camera_meta_transformation(
                        main_stream_profile
                            .get_extrinsics_to(&stream_profile)
                            .map_err(|e| gst::error_msg!(gst::StreamError::Failed, ["{}", e]))?,
                    ),
                );
            }
        }

        Ok(extrinsics)
    }

    /// Convert RealSense Extrinsics into CameraMeta Transformation, which is used for creation of camera_meta::Extrinsics.
    ///
    /// # Arguments
    /// * `rs2_extrinsics` - Realsense extrinsics to convert.
    ///
    /// # Returns
    /// * `camera_meta::Transformation` containing the converted transformation.
    fn rs2_extrinsics_to_camera_meta_transformation(
        rs2_extrinsics: rs2::extrinsics::Extrinsics,
    ) -> camera_meta::Transformation {
        camera_meta::Transformation::new(
            camera_meta::Translation::from(rs2_extrinsics.translation),
            camera_meta::RotationMatrix::from(rs2_extrinsics.rotation),
        )
    }

    /// Extract the depth scale from RealSense Sensors.
    ///
    /// # Arguments
    /// * `sensors` - List of active RealSense sensors.
    ///
    /// # Returns
    /// * `f32` containing the depth scale, in metres. Default value of 0.001 is returned if depth sensor is not active.
    fn get_depth_scale(sensors: rs2::sensor::SensorList) -> f32 {
        for i in 0..sensors.count().unwrap() {
            let sensor = sensors.create_sensor(i).unwrap();
            let depth_scale = sensor.get_depth_scale();
            if let Ok(depth_scale) = depth_scale {
                // Return the depth scale as soon as it is found in sensors.
                return depth_scale;
            }
        }
        // If depth scale cannot be found (depth stream is not active), return the default depth scale.
        DEFAULT_DEPTH_SCALE
    }

    /// Attach Cap'n Proto serialised CameraMeta to `output_buffer`.
    ///
    /// # Arguments
    /// * `push_src` - Representation of `realsensesrc` element.
    /// * `output_buffer` - The output buffer to which the ImuSamples will be attached.
    /// * `camera_meta` - Serialised CameraMeta to attach to the `output_buffer`.
    ///
    /// # Returns
    /// * `Ok()` on success.
    /// * `Err(RealsenseError)` on failure.
    fn attach_camera_meta(
        &self,
        output_buffer: &mut gst::Buffer,
        camera_meta: Vec<u8>,
        duration: gst::ClockTime,
    ) -> Result<(), ErrorMessage> {
        // Form a gst buffer out of mutable slice
        let mut buffer = gst::buffer::Buffer::from_mut_slice(camera_meta);
        // Get mutable reference to the buffer
        let buffer_mut_ref = buffer.get_mut().ok_or_else(|| {
            gst::error_msg!(
                gst::StreamError::Failed,
                [
                    "Cannot get mutable reference to the buffer for {}",
                    STREAM_ID_CAMERAMETA
                ]
            )
        })?;

        buffer_mut_ref.set_duration(duration);

        // Attach the camera_meta buffer and tag it adequately
        rgbd::attach_aux_buffer_and_tag(
            output_buffer.get_mut().ok_or_else(|| {
                gst::error_msg!(
                    gst::StreamError::Failed,
                    [
                        "Cannot get mutable reference to the main buffer while attaching {}",
                        STREAM_ID_CAMERAMETA
                    ]
                )
            })?,
            &mut buffer,
            STREAM_ID_CAMERAMETA,
        )?;

        Ok(())
    }

    /// Determine the main stream, while taking into account the priority `depth > infra1 > infra2 > color`, and return the corresponding ID.
    ///
    /// # Arguments
    /// * `streams` - Struct containing enabled streams.
    ///
    /// # Returns
    /// * `&str` containing the ID of the main stream.
    fn determine_main_stream(streams: &EnabledStreams) -> StreamId {
        if streams.depth {
            StreamId::Depth
        } else if streams.infra1 {
            StreamId::Infra1
        } else if streams.infra2 {
            StreamId::Infra2
        } else {
            StreamId::Color
        }
    }

    /// Determines whether a specific stream id is enabled in `streams`.
    ///
    /// # Arguments
    /// * `stream_id` - Stream ID.
    /// * `streams` - Struct containing the enabled streams.
    ///
    /// # Returns
    /// * `true` if a stream with the `stream_id` is enabled, `false` otherwise .
    fn is_stream_enabled(stream_id: StreamId, streams: &EnabledStreams) -> bool {
        (stream_id == StreamId::Depth && streams.depth)
            || (stream_id == StreamId::Infra1 && streams.infra1)
            || (stream_id == StreamId::Infra2 && streams.infra2)
            || (stream_id == StreamId::Color && streams.color)
    }

    /// Attempt to find the frame for the given `stream_id` in the Vector of frames extracted from the
    /// RealSense camera. This function returns `None` on missing or erroneous frames.
    /// # Arguments
    /// * `frames` - A vector of frames extracted from librealsense.
    /// * `stream_type` - The type of the stream to look for.
    /// * `stream_id` - The id of the frame you wish to find.
    fn find_frame_with_id(
        frames: &[Frame],
        stream_type: rs2::rs2_stream,
        stream_id: i32,
    ) -> Option<&Frame> {
        frames.iter().find(|f| match f.get_stream_profile() {
            Ok(profile) => match profile.get_data() {
                Ok(data) => {
                    data.stream == stream_type
                        && if stream_id == -1 {
                            true
                        } else {
                            data.index == stream_id
                        }
                }
                _ => false,
            },
            _ => false,
        })
    }

    /// Stop RealSense pipeline and reset internals (except for settings).
    /// # Returns
    /// * Ok() on success.
    /// * Err(gst::ErrorMessage) if RealSense pipeline could not be stopped.
    /// # Panics
    /// * If RealSense pipeline is already stopped, which should never occur.
    fn stop_rs_and_reset(&self) -> Result<(), gst::ErrorMessage> {
        let mut internals = self.internals.lock().unwrap();

        internals.pipeline.as_ref().unwrap().stop().map_err(|e| {
            gst::error_msg!(
                gst::StreamError::Failed,
                ["RealSense pipeline could not be stopped: {:?}", e]
            )
        })?;

        let settings = self.settings.read().unwrap();
        if let Some(serial) = &settings.serial {
            // Hardware reset the device if it was used during streaming
            let device = Self::find_device(internals.context.as_ref().unwrap(), serial)?;
            if let Err(err) = device.hardware_reset() {
                gst_warning!(CAT, "Could not perform a hardware reset: {}", err);
            }
        }

        *internals = Default::default();
        Ok(())
    }
}

impl ElementImpl for RealsenseSrc {
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "Realsense Source",
                "Source/RGB-D/Realsense",
                "Stream `video/rgbd` from a RealSense device",
                "Niclas Overby <niclas.overby@aivero.com>, \
                 Andrej Orsula <andrej.orsula@aivero.com>, \
                 Tobias Morell <tobias.morell@aivero.com>, \
                 Jimmi Christensen <jimmi.christensen@aivero.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<[gst::PadTemplate; 1]> = Lazy::new(|| {
            let src_caps = gst::Caps::new_simple(
                "video/rgbd",
                &[(
                    "framerate",
                    &gst::FractionRange::new(
                        gst::Fraction::new(MIN_FRAMERATE, 1),
                        gst::Fraction::new(MAX_FRAMERATE, 1),
                    ),
                )],
            );
            [gst::PadTemplate::new(
                "src",
                gst::PadDirection::Src,
                gst::PadPresence::Always,
                &src_caps,
            )
            .unwrap()]
        });

        PAD_TEMPLATES.as_ref()
    }
}

impl GstObjectImpl for RealsenseSrc {}
impl ObjectImpl for RealsenseSrc {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_format(gst::Format::Time);
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<[glib::ParamSpec; 19]> = Lazy::new(|| {
            [
                glib::ParamSpecString::new(
                    "serial",
                    "Serial Number",
                    "Serial number of a realsense device. If unchanged or empty,
                     `rosbag-location` is used to locate a file to play from.",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "rosbag-location",
                    "Rosbag File Location",
                    "Location of a rosbag file to play from. If unchanged or empty, physical
                     device specified by `serial` is used.",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "config",
                    "Realsense config json string",
                    "The string to configure RealSense device from. This property applies only \
                     if `serial` is specified.",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "enable-depth",
                    "Enable Depth",
                    "Enables depth stream.",
                    DEFAULT_ENABLE_DEPTH,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "enable-infra1",
                    "Enable Infra1",
                    "Enables infra1 stream.",
                    DEFAULT_ENABLE_INFRA1,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "enable-infra2",
                    "Enable Infra2",
                    "Enables infra2 stream.",
                    DEFAULT_ENABLE_INFRA2,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "enable-color",
                    "Enable Color",
                    "Enables color stream.",
                    DEFAULT_ENABLE_COLOR,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt::new(
                    "depth-width",
                    "Depth Width",
                    "Width of the depth and infra1/infra2 frames.",
                    DEPTH_MIN_WIDTH,
                    DEPTH_MAX_WIDTH,
                    DEFAULT_DEPTH_WIDTH,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt::new(
                    "depth-height",
                    "Depth Height",
                    "Height of the depth and infra1/infra2 frames.",
                    DEPTH_MIN_HEIGHT,
                    DEPTH_MAX_HEIGHT,
                    DEFAULT_DEPTH_HEIGHT,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt::new(
                    "color-width",
                    "Color Width",
                    "Width of the color frame.",
                    COLOR_MIN_WIDTH,
                    COLOR_MAX_WIDTH,
                    DEFAULT_COLOR_WIDTH,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt::new(
                    "color-height",
                    "Color Height",
                    "Height of the color frame.",
                    COLOR_MIN_HEIGHT,
                    COLOR_MAX_HEIGHT,
                    DEFAULT_COLOR_HEIGHT,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecInt::new(
                    "framerate",
                    "Framerate",
                    "Common framerate of the selected streams.",
                    MIN_FRAMERATE,
                    MAX_FRAMERATE,
                    DEFAULT_FRAMERATE,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "loop-rosbag",
                    "Loop Rosbag",
                    "Enables looping of playing from rosbag recording specified by
                     `rosbag-location` property. This property applies only if
                     `rosbag-location` and no `serial` are specified.",
                    DEFAULT_LOOP_ROSBAG,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecUInt::new(
                    "wait-for-frames-timeout",
                    "Wait For Frames Timeout",
                    "Timeout used while waiting for frames from a RealSense device in
                     milliseconds.",
                    std::u32::MIN,
                    std::u32::MAX,
                    DEFAULT_PIPELINE_WAIT_FOR_FRAMES_TIMEOUT,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "include-per-frame-metadata",
                    "Include Per Frame Metadata",
                    "Attempts to include librealsense2's per-frame metadata as an additional
                     buffer on the main buffer. Per-frame metadata is silently ignored if it
                     cannot be fetched from the librealsense2 frames.",
                    DEFAULT_ENABLE_METADATA,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "real-time-rosbag-playback",
                    "Real Time Rosbag Playback",
                    "Determines whether to stream from the file the same way it was recorded.
                     If set to false, streaming rate will be determined based on the negotiated
                     framerate or it will be as fast as possible if downstream elements are
                     async.",
                    DEFAULT_REAL_TIME_ROSBAG_PLAYBACK,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecBoolean::new(
                    "attach-camera-meta",
                    "Attach Camera Meta",
                    "If enabled, `video/rgbd` will also contain the meta associated with
                     RealSense camera, such as intrinsics and extrinsics.",
                    DEFAULT_ATTACH_CAMERA_META,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecEnum::new(
                    "align-to",
                    "The stream to align to",
                    "The name of the stream to align to (target). Supported values are 'depth'
                     and 'color'. Note that aligning of 'infra1' and 'infra2' streams to
                     'color' is not supported.",
                    StreamId::static_type(),
                    StreamId::default() as i32,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecEnum::new(
                    "log-level",
                    "The librealsense log level",
                    "The log level to set librealsense to use",
                    LogLevel::static_type(),
                    LogLevel::default() as i32,
                    glib::ParamFlags::READWRITE,
                ),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        obj: &Self::Type,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        let mut settings = self.settings.write().unwrap();

        gst_info!(
            CAT,
            obj: obj,
            "Changing property '{}' to {:?}",
            pspec.name(),
            value
        );
        match pspec.name() {
            "serial" => match value.get().unwrap() {
                Some(serial) => {
                    settings.serial = Some(serial);
                    obj.set_live(true);
                }
                None => {
                    gst_warning!(
                        CAT,
                        obj: obj,
                        "`serial` property not set, setting from {:?} to None",
                        settings.serial
                    );
                }
            },
            "rosbag-location" => match value.get().unwrap() {
                Some(mut rl) => {
                    expand_tilde_as_home_dir(&mut rl);
                    settings.rosbag_location = Some(rl);
                    obj.set_live(settings.real_time_rosbag_playback);
                }
                None => {
                    gst_warning!(
                        CAT,
                        obj: obj,
                        "`rosbag-location` property not set, setting from {:?} to None",
                        settings.rosbag_location
                    );
                }
            },
            "config" => settings.config = value.get().ok(),
            "enable-depth" => settings.streams.enabled_streams.depth = value.get().unwrap(),
            "enable-infra1" => settings.streams.enabled_streams.infra1 = value.get().unwrap(),
            "enable-infra2" => settings.streams.enabled_streams.infra2 = value.get().unwrap(),
            "enable-color" => settings.streams.enabled_streams.color = value.get().unwrap(),
            "depth-width" => settings.streams.depth_resolution.width = value.get().unwrap(),
            "depth-height" => settings.streams.depth_resolution.height = value.get().unwrap(),
            "color-width" => settings.streams.color_resolution.width = value.get().unwrap(),
            "color-height" => settings.streams.color_resolution.height = value.get().unwrap(),
            "framerate" => settings.streams.framerate = value.get().unwrap(),
            "loop-rosbag" => settings.loop_rosbag = value.get().unwrap(),
            "wait-for-frames-timeout" => settings.wait_for_frames_timeout = value.get().unwrap(),
            "include-per-frame-metadata" => {
                settings.include_per_frame_metadata = value.get().unwrap()
            }
            "real-time-rosbag-playback" => {
                settings.real_time_rosbag_playback = value.get().unwrap();
                obj.set_live(settings.real_time_rosbag_playback);
            }
            "attach-camera-meta" => settings.attach_camera_meta = value.get().unwrap(),
            "align-to" => settings.align_to = value.get().unwrap(),
            "log-level" => settings.log_level = value.get().unwrap(),
            _ => unreachable!(),
        };
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        let settings = &self.settings.read().unwrap();

        match pspec.name() {
            "serial" => settings.serial.to_value(),
            "rosbag-location" => settings.rosbag_location.to_value(),
            "config" => settings.config.to_value(),
            "enable-depth" => settings.streams.enabled_streams.depth.to_value(),
            "enable-infra1" => settings.streams.enabled_streams.infra1.to_value(),
            "enable-infra2" => settings.streams.enabled_streams.infra2.to_value(),
            "enable-color" => settings.streams.enabled_streams.color.to_value(),
            "depth-width" => settings.streams.depth_resolution.width.to_value(),
            "depth-height" => settings.streams.depth_resolution.height.to_value(),
            "color-width" => settings.streams.color_resolution.width.to_value(),
            "color-height" => settings.streams.color_resolution.height.to_value(),
            "framerate" => settings.streams.framerate.to_value(),
            "loop-rosbag" => settings.loop_rosbag.to_value(),
            "wait-for-frames-timeout" => settings.wait_for_frames_timeout.to_value(),
            "include-per-frame-metadata" => settings.include_per_frame_metadata.to_value(),
            "real-time-rosbag-playback" => settings.real_time_rosbag_playback.to_value(),
            "attach-camera-meta" => settings.attach_camera_meta.to_value(),
            "align-to" => settings.align_to.to_value(),
            "log-level" => settings.log_level.to_value(),
            _ => unimplemented!("Property is not implemented"),
        }
    }
}

/// Helper function that replaces "~/" at the beginning of `path` with "$HOME/",
/// while `path` remains unchanged if it does not start with "~/".
fn expand_tilde_as_home_dir(path: &mut String) {
    if path.starts_with("~/") {
        let home_path = std::env::var("HOME")
        .expect("k4asrc: $HOME must be specified if a path for property is specified with \"~\" (tilde).");
        path.replace_range(..1, &home_path);
    }
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "realsensesrc",
        gst::Rank::None,
        RealsenseSrc::type_(),
    )
}
