use glib::*;
use gst::glib;
use gst::subclass::prelude::*;
use gst::{prelude::*, TagList};
use gst_depth_meta::rgbd;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

use crate::common::*;

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "rgbddemux",
        gst::DebugColorFlags::empty(),
        Some("RGB-D Demuxer"),
    )
});

/// Default value for to `distribute-timestamps` property
const DEFAULT_DISTRIBUTE_TIMESTAMPS: bool = true;

/// A struct that identifies a stream.
struct StreamIdentifier {
    /// The id of the stream.
    stream_id: String,
    /// The group id of the stream.
    group_id: gst::GroupId,
}

impl StreamIdentifier {
    fn build_stream_start_event(&self, stream_name: &str) -> gst::Event {
        gst::event::StreamStart::builder(&format!("{}/{}", self.stream_id, stream_name))
            .group_id(self.group_id)
            .build()
    }
}

/// A handle on the pad, which contains information related to the pad.
#[derive(Debug)]
struct DemuxPad {
    /// The actual pad.
    pad: gst::Pad,
    // todo: Remove this flag - the stream-start event is sticky, therefore we can get this state from the pad itself
    /// A flag to indicate whether or not we have sent the "stream-start" event on the pad.
    pushed_stream_start: bool,
}

impl DemuxPad {
    /// Creates a new DemuxPad for the given `pad`.
    /// # Arguments
    /// * `pad` - The pad to create a handle for.
    /// # Returns
    /// A new instance of [DemuxPad](struct.DemuxPad.html) for the pad.
    pub fn new(pad: gst::Pad) -> Self {
        Self {
            pad,
            pushed_stream_start: false,
        }
    }

    /// Deactive and remove `self` (pad) from `element`.
    /// # Arguments
    /// * `element` - The element that represents `rgbddemux` in GStreamer.
    /// # Panics
    /// * If one of the unneeded pads cannot be deactivated or removed from the element.
    pub fn deactivate_and_remove_from_element(&self, element: &RgbdDemuxObject) {
        self.pad.set_active(false).unwrap();
        element.remove_pad(&self.pad).unwrap();
    }
}

/// Hashmap of source pads of the demuxer, where key is the name of the stream flowing through
/// that pad.
type SrcPads = HashMap<String, DemuxPad>;

/// A struct containing properties of `rgbddemux` element
struct Settings {
    /// Analogous to `distribute-timestamps` property
    distribute_timestamps: bool,
}

/// A struct representation of the `rgbddemux` element.
pub struct RgbdDemux {
    /// Settings based on properties of the element.
    settings: RwLock<Settings>,
    /// List of source pads on this demuxer.
    src_pads: RwLock<SrcPads>,
    /// Utility struct that handles GstFlowReturn combinations with multiple source pads.
    flow_combiner: Mutex<gst_base::UniqueFlowCombiner>,
    /// Utility struct that contains stream and group id, used when pushing stream-start events.
    stream_identifier: Mutex<Option<StreamIdentifier>>,
    /// Utility struct that groups received Tags
    tags_not_sent: Mutex<TagList>,
    pad_to_send_tags_on: Mutex<Option<gst::Pad>>,
    sink_pad: gst::Pad,
}

glib::wrapper! {
    pub struct RgbdDemuxObject(ObjectSubclass<RgbdDemux>)
        @extends gst::Element, gst::Object;
}

#[glib::object_subclass]
impl ObjectSubclass for RgbdDemux {
    const NAME: &'static str = "rgbddemux";
    type Type = RgbdDemuxObject;
    type ParentType = gst::Element;
    fn with_class(klass: &Self::Class) -> Self {
        let templ = klass.pad_template("sink").unwrap();
        let sink_pad = gst::Pad::builder_with_template(&templ, Some("sink"))
            .chain_function(|pad, parent, buffer| {
                Self::catch_panic_pad_function(
                    parent,
                    || Err(gst::FlowError::Error),
                    |this, element| this.sink_chain(pad, element, buffer),
                )
            })
            .event_function(|pad, parent, event| {
                Self::catch_panic_pad_function(
                    parent,
                    || false,
                    |this, element| this.sink_event(pad, element, event),
                )
            })
            .build();

        Self {
            settings: RwLock::new(Settings {
                distribute_timestamps: DEFAULT_DISTRIBUTE_TIMESTAMPS,
            }),
            src_pads: RwLock::new(HashMap::new()),
            flow_combiner: Mutex::new(gst_base::UniqueFlowCombiner::new()),
            stream_identifier: Mutex::new(None),
            tags_not_sent: Mutex::new(TagList::new()),
            pad_to_send_tags_on: Mutex::new(None),
            sink_pad,
        }
    }
}

impl ElementImpl for RgbdDemux {
    fn change_state(
        &self,
        element: &Self::Type,
        transition: gst::StateChange,
    ) -> Result<gst::StateChangeSuccess, gst::StateChangeError> {
        #[allow(clippy::single_match)]
        match transition {
            gst::StateChange::PausedToReady => {
                self.reset();
            }
            _ => {}
        }

        // Chain up parent implementation
        self.parent_change_state(element, transition)
    }

    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "RGB-D Demuxer",
                "Demuxer/RGB-D",
                "Demuxes a single `video/rgbd` stream into multiple elementary streams",
                "Raphael Duerscheid <rd@aivero.com>, Andrej Orsula <andrej.orsula@aivero.com>,
                 Tobias Morell <tobias.morell@aivero.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<[gst::PadTemplate; 2]> = Lazy::new(|| {
            let mut src_caps = gst::Caps::new_simple("video/x-raw", &[]);
            {
                let src_caps = src_caps.get_mut().unwrap();
                src_caps.append(gst::Caps::new_simple("meta/x-klv", &[("parsed", &true)]));
                src_caps.append(gst::Caps::new_simple("image/jpeg", &[]));
            }

            [
                gst::PadTemplate::new(
                    "src_%s",
                    gst::PadDirection::Src,
                    gst::PadPresence::Sometimes,
                    &src_caps,
                )
                .expect("rgbddemux: Failed to add 'src_%s' pad template"),
                gst::PadTemplate::new(
                    "sink",
                    gst::PadDirection::Sink,
                    gst::PadPresence::Always,
                    &gst::Caps::new_simple("video/rgbd", &[]),
                )
                .expect("rgbddemux: Failed to add 'sink' pad template"),
            ]
        });

        PAD_TEMPLATES.as_ref()
    }
}

impl RgbdDemux {
    /// Called whenever an event is received at the sink pad. CAPS and stream start events will
    /// be handled locally, all other events are send further downstream.
    /// # Arguments
    /// * `element` - The element that represents the `rgbddemux` in GStreamer.
    /// * `event` - The event that should be handled.
    fn sink_event(&self, _pad: &gst::Pad, element: &RgbdDemuxObject, event: gst::Event) -> bool {
        use gst::EventView;
        match event.view() {
            EventView::Caps(caps) => {
                gst_debug!(CAT, obj: element, "Got a new caps event: {:?}", caps);
                // Create src pads according to the received upstream CAPS
                match self.create_src_pads_from_sink_caps(element, caps.caps()) {
                    Ok(_) => {
                        gst_debug!(CAT, "Caps successfully renegotiated");
                        true
                    }
                    Err(e) => {
                        gst_error!(CAT, obj: element, "{}", e);
                        false
                    }
                }
            }
            EventView::StreamStart(stream_start) => {
                gst_debug!(CAT, "Got a stream start event {:?}", stream_start);
                let stream_identifier = StreamIdentifier {
                    stream_id: stream_start.stream_id().to_string(),
                    group_id: stream_start.group_id().unwrap_or_else(gst::GroupId::next),
                };

                // Pass stream start event on all existing src pads
                self.push_stream_start_on_all_pads(&stream_identifier);

                // Update internals with the new stream identifier
                *self.stream_identifier.lock().unwrap() = Some(stream_identifier);

                // Accept any StreamStart event
                true
            }
            EventView::Tag(tags) => {
                if let Some(pad) = &*self.pad_to_send_tags_on.lock().unwrap() {
                    let tag_event = gst::event::Tag::new(tags.tag_owned());
                    if !pad.push_event(tag_event) {
                        gst_warning!(CAT, "Could not send Tag event for stream");
                    }
                } else {
                    // If we don't have any pads to push the tags to, the we will store them
                    // and send them later once pads become available.
                    let mut tags_not_sent = self.tags_not_sent.lock().unwrap();
                    let tags_not_sent = tags_not_sent.make_mut();
                    tags_not_sent.insert(tags.tag(), gst::TagMergeMode::Append);
                }

                true
            }

            _ => self.sink_pad.event_default(Some(element), event),
        }
    }

    /// Create additional src pads, which happens as a result of a CAPS renegotiation.
    /// # Arguments
    /// * `element` - The element that represents `rgbddemux` in GStreamer.
    /// * `rgbd_caps` - The sink CAPS that we should create src pads for.
    fn create_src_pads_from_sink_caps(
        &self,
        element: &RgbdDemuxObject,
        rgbd_caps: &gst::CapsRef,
    ) -> Result<(), gst::ErrorMessage> {
        gst_debug!(
            CAT,
            "Creating new src pads based on the negotiated upstream CAPS:\n{:?}\n\n",
            rgbd_caps
        );

        // Extract the `video/rgbd` caps fields from gst::CapsRef
        let rgbd_caps = rgbd_caps.structure(0).ok_or_else(|| {
            gst::error_msg!(
                gst::CoreError::Tag,
                ["Invalid `video/rgbd` caps for creation of additional src pads",]
            )
        })?;

        // Determine what streams are contained within the caps
        let streams = rgbd_caps.get::<gst::Array>("streams").map_err(|err| {
            gst::error_msg!(
                gst::CoreError::Tag,
                [
                    "No `streams` field detected in `video/rgbd` caps: {:?}",
                    err
                ]
            )
        })?;

        let streams: Vec<&str> = streams
            .as_slice()
            .iter()
            .map(|s| s.get::<&str>().unwrap())
            .collect();

        if streams.is_empty() {
            return Err(gst::error_msg!(
                gst::CoreError::Tag,
                ["Cannot detect any streams in `video/rgbd` caps under field `streams`",]
            ));
        }

        // Get a common framerate for all streams
        let common_framerate = rgbd_caps.get::<gst::Fraction>("framerate").map_err(|err| {
            gst::error_msg!(
                gst::CoreError::Tag,
                [
                    "Cannot detect any `framerate` in `video/rgbd` caps: {:?}",
                    err
                ]
            )
        })?;

        let mut src_pads = self.src_pads.write().unwrap();

        // Remove pads that are no longer needed for the new CAPS
        Self::remove_unneeded_pads(element, &mut src_pads, streams.as_slice());

        // Iterate over all streams, find their caps and push a CAPS negotiation event
        let mut flow_combiner = self.flow_combiner.lock().unwrap();
        for stream_name in streams.iter() {
            // Determine the appropriate caps for the stream
            let new_pad_caps =
                self.extract_stream_caps(stream_name, rgbd_caps, common_framerate)?;

            self.create_new_src_pad(
                element,
                &mut src_pads,
                &mut flow_combiner,
                stream_name,
                &new_pad_caps,
            );
        }

        element.no_more_pads();
        Ok(())
    }

    /// Extract CAPS for the given stream from the given rgbd_caps.
    /// # Arguments
    /// * `stream_name` - The name of the stream to extract CAPS for, e.g. `depth`.
    /// * `rgbd_caps` - A reference to the `video/rgbd` CAPS, from which the stream's CAPS should be extracted.
    /// * `common_framerate` - The framerate of all the streams.
    fn extract_stream_caps(
        &self,
        stream_name: &str,
        rgbd_caps: &gst::StructureRef,
        common_framerate: gst::Fraction,
    ) -> Result<gst::Caps, gst::ErrorMessage> {
        // Return "meta/x-klv" if we are dealing with metadata stream
        if stream_name.contains("meta") {
            return Ok(gst::Caps::new_simple("meta/x-klv", &[("parsed", &true)]));
        };
        // Get the format of a stream
        let stream_format = rgbd_caps
            .get::<String>(&format!("{}_format", stream_name))
            .map_err(|err| {
                gst::error_msg!(
                    gst::CoreError::Tag,
                    [
                        "Cannot detect any `format` in `video/rgbd` caps for `{}` stream: {:?}",
                        stream_name,
                        err
                    ]
                )
            })?;

        // Return "image/jpeg" CAPS if the format is MJPG
        if stream_format.contains("jpeg") {
            return Ok(gst::Caps::new_simple("image/jpeg", &[]));
        }

        // Get the width of a stream
        let stream_width = rgbd_caps
            .get::<i32>(&format!("{}_width", stream_name))
            .map_err(|err| {
                gst::error_msg!(
                    gst::CoreError::Tag,
                    [
                        "Cannot detect any `width` in `video/rgbd` caps for `{}` stream: {:?}",
                        stream_name,
                        err
                    ]
                )
            })?;

        // Get the height of a stream
        let stream_height = rgbd_caps
            .get::<i32>(&format!("{}_height", stream_name))
            .map_err(|err| {
                gst::error_msg!(
                    gst::CoreError::Tag,
                    [
                        "Cannot detect any `height` in `video/rgbd` caps for `{}` stream: {:?}",
                        stream_name,
                        err
                    ]
                )
            })?;

        // Create caps for the new "video/x-raw" src pad
        Ok(gst::Caps::new_simple(
            "video/x-raw",
            &[
                ("format", &stream_format),
                ("width", &stream_width),
                ("height", &stream_height),
                ("framerate", &common_framerate),
            ],
        ))
    }

    /// Remove pads from `src_pads` that are no longer needed in a set of `allowed_streams`.
    /// Note that this function removes the pads both from element and `src_pads`.
    /// # Arguments
    /// * `element` - The element that represents `rgbddemux` in GStreamer.
    /// * `src_pads` - List of current source pads.
    /// * `allowed_streams` - List of allowed streams, e.g. based on upstream CAPS.
    /// # Panics
    /// * If one of the unneeded pads cannot be deactivated or removed from the element.
    fn remove_unneeded_pads(
        element: &RgbdDemuxObject,
        src_pads: &mut SrcPads,
        allowed_streams: &[&str],
    ) {
        src_pads.retain(|stream_name, src_pad| {
            if !allowed_streams
                .iter()
                .any(|&caps_stream| caps_stream == stream_name)
            {
                gst_debug!(
                    CAT,
                    obj: element,
                    "Removing pad for {} stream as it is no longer needed in the newly negotiated streams {:?}",
                    stream_name, allowed_streams
                );

                // Deactivate and remove the pad from the element
                src_pad.deactivate_and_remove_from_element(element);

                false
            } else {
                true
            }
        });
    }

    /// Create a new src pad on the `rgbddemux` for the stream with the given name.
    /// This function won't create a pad if it already exists.
    /// # Arguments
    /// * `element` - The element that represents the `rgbddemux` in GStreamer.
    /// * `src_pads` - A mutable reference to the collection of pads currently present on the `rgbddemux`.
    /// * `flow_combiner` - A mutable reference to the flow combiner that drives the `rgbddemux`.
    /// * `stream_name` - The name of the stream to create a src pad for.
    /// * `new_pad_caps` - CAPS of the new pad's stream, which are used to send CAPS event downstream.
    /// # Panics
    /// * If the new pad cannot be added to `element`.
    /// * If the new pad cannot be activated.
    fn create_new_src_pad(
        &self,
        element: &RgbdDemuxObject,
        src_pads: &mut SrcPads,
        flow_combiner: &mut gst_base::UniqueFlowCombiner,
        stream_name: &str,
        new_pad_caps: &gst::Caps,
    ) {
        gst_debug!(
            CAT,
            obj: element,
            "Creating new src pad for {} stream",
            stream_name
        );

        // Create naming for the src pad according to the stream
        let new_src_pad_name = &format!("src_{}", stream_name);

        // Do not create a new pad if it already exists
        if src_pads.contains_key(stream_name) {
            gst_info!(
                CAT,
                obj: element,
                "Pad `{}` was already created during previous CAPS negotiation with upstream element",
                new_src_pad_name
            );

            // Check if the pad requires new CAPS re-negotiation with downstream element
            let existing_pad = src_pads.get_mut(stream_name).unwrap();
            if let Some(current_caps) = &existing_pad.pad.current_caps() {
                if current_caps != new_pad_caps {
                    // Deactivate and mark the pad for reconfiguration
                    existing_pad.pad.set_active(false).unwrap();
                    existing_pad.pad.mark_reconfigure();

                    // Prepare the pad for streaming again, i.e. send events and activate it
                    existing_pad.pad.set_active(true).unwrap();
                    self.prepare_new_pad(existing_pad, stream_name, new_pad_caps);
                }
            }
            return;
        }

        // Create the src pad based on template
        let pad = gst::Pad::builder_with_template(
            &element.pad_template("src_%s").unwrap(),
            Some(new_src_pad_name),
        )
        .flags(gst::PadFlags::FIXED_CAPS)
        .build();

        // Add the pad to the element
        element.add_pad(&pad).unwrap();
        pad.set_active(true).unwrap();

        let mut send_pad = self.pad_to_send_tags_on.lock().unwrap();
        if stream_name != "dddqmeta" && send_pad.is_none() {
            // Push tags that were received before we had any pads.
            let tags_not_sent = self.tags_not_sent.lock().unwrap();
            let tag_event = gst::event::Tag::new(tags_not_sent.clone());
            if !pad.push_event(tag_event) {
                gst_warning!(CAT, "Could not push tags to {}", stream_name);
            }
            *send_pad = Some(pad.clone());
        }

        // Create a DemuxPad from it
        let mut pad_handle = DemuxPad::new(pad);

        // Prepare new pad for streaming, i.e. send events and activate it
        self.prepare_new_pad(&mut pad_handle, stream_name, new_pad_caps);

        // Add the new pad to the internals
        flow_combiner.add_pad(&pad_handle.pad);
        src_pads.insert(stream_name.to_string(), pad_handle);

        gst_debug!(
            CAT,
            "New pad for {} stream successfuly created",
            stream_name
        );
    }

    /// Send stream-start and CAPS events downstream, after which the pad is activated.
    /// # Arguments
    /// * `pad_handle` - A mutable reference to the handle for the new pad.
    /// * `stream_name` - The name of the stream to create a src pad for.
    /// * `new_pad_caps` - CAPS of the new pad's stream, which are used to send CAPS event downstream.
    /// # Panics
    /// * If the new pad cannot be activated.
    fn prepare_new_pad(
        &self,
        pad_handle: &mut DemuxPad,
        stream_name: &str,
        new_pad_caps: &gst::Caps,
    ) {
        // Push stream-start event to indicate beginning of the stream
        if let Some(stream_identifier) = self.stream_identifier.lock().unwrap().as_ref() {
            Self::push_stream_start(pad_handle, stream_name, stream_identifier);
        }

        // Push CAPS event, so that downstream element know what data they're dealing with
        gst_debug!(CAT, "Pushing a new caps event for {} stream", stream_name);
        pad_handle
            .pad
            .push_event(gst::event::Caps::new(new_pad_caps));
    }

    /// Called whenever a new buffer is passed to the sink pad. This function splits the buffer in
    /// to multiple buffer, which are pushed on their corresponding src pad.
    /// # Arguments
    /// * `element` - The element that represents the `rgbddemux` in GStreamer.
    /// * `main_buffer` - The buffer that was received on the sink pad.
    fn sink_chain(
        &self,
        _: &gst::Pad,
        element: &RgbdDemuxObject,
        main_buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        // Distribute the timestamp of the main buffer to the auxiliary buffers, if enabled
        let distrubute_timestamps = self.settings.read().unwrap().distribute_timestamps;

        let common_pts = main_buffer.pts();
        let common_dts = main_buffer.dts();
        let common_duration = main_buffer.duration();
        gst_trace!(
            CAT,
            obj: element,
            "Pushing buffers to their corresponding pads",
        );

        // Go through all buffers in order to extract them and push to the corresponding src pads
        let src_pads = self.src_pads.read().unwrap();

        let mut flow_combiner = self.flow_combiner.lock().unwrap();
        for mut buffer in rgbd::get_all_buffers(main_buffer) {
            let buffer = buffer.make_mut();
            if distrubute_timestamps {
                buffer.set_dts(common_dts);
                buffer.set_pts(common_pts);
                buffer.set_duration(common_duration);
            }

            rgbd::remove_aux_buffers(buffer);
            flow_combiner.update_flow(self.push_buffer_to_corresponding_pad(
                element,
                &src_pads,
                buffer.copy(),
            ))?;
        }
        Ok(gst::FlowSuccess::Ok)
    }

    /// Push the given buffer to the src pad that was allocated for it.
    /// # Arguments
    /// * `element` - The element that represents the `rgbddemux` in GStreamer.
    /// * `src_pads` - A hash map that associates buffer title tags with their corresponding pad.
    /// * `buffer` - The buffer that should be pushed further downstream.
    fn push_buffer_to_corresponding_pad(
        &self,
        element: &RgbdDemuxObject,
        src_pads: &HashMap<String, DemuxPad>,
        buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        // Extract tag title from the buffer
        let tag_title = rgbd::get_tag(&buffer).map_err(|e| {
            gst_warning!(CAT, obj: element, "Failed to get buffer tag: {}", e);
            gst::FlowError::Error
        })?;

        // Match the tag title with a corresponding src pad
        if let Some(src_pad) = src_pads.get(&tag_title) {
            // Do not attempt to push buffers, if pad is not linked
            if !src_pad.pad.is_linked() {
                gst_debug!(
                    CAT,
                    obj: element,
                    "Buffer {} is not pushed because its pad is not linked",
                    tag_title
                );
                return Ok(gst::FlowSuccess::Ok);
            }

            // Attempt to push a buffer to the pad
            gst_trace!(
                CAT,
                obj: element,
                "Pushing buffer for stream {} to the corresponding pad",
                tag_title
            );
            src_pad.pad.push(buffer)
        } else {
            // We cannot push the buffer if we do not know its destination.
            // Instead of throwing an exception, provide a warning and process all other buffers.
            gst_warning!(
                CAT,
                obj: element,
                "Cannot push buffer tagged as {} because no corresponding pad was \
                 created. Caps: {:?}",
                tag_title,
                self.sink_pad.current_caps(),
            );
            Ok(gst::FlowSuccess::Ok)
        }
    }

    /// Push a stream-start event on the given pad, so that downstream can expect data flow.
    /// This function makes sure that the pad sends only one stream-start event.
    /// # Arguments
    /// * `src_pad` - The pad to push the event on.
    /// * `stream_name` - A unique name of the stream to push the event for.
    /// * `stream_identifier` - A stream identifier that uniquely identifies the current stream.
    fn push_stream_start(
        src_pad: &mut DemuxPad,
        stream_name: &str,
        stream_identifier: &StreamIdentifier,
    ) {
        if src_pad.pushed_stream_start {
            gst_debug!(CAT, "Stream start event for stream {} was already pushed this streaming session, skipping", stream_name);
            return;
        }

        let stream_start_event = stream_identifier.build_stream_start_event(stream_name);
        src_pad.pushed_stream_start = src_pad.pad.push_event(stream_start_event);
        if !src_pad.pushed_stream_start {
            gst_warning!(
                CAT,
                "Could not send stream start event for stream {}",
                stream_name
            );
        }
    }

    /// Push the "stream-start" event on all pads in `self.src_pads` that have not yet seen a
    /// "stream-start" event.
    /// # Arguments
    /// * `stream_identifier` - An instance of the [StreamIdentifier](struct.StreamIdentifier.html) struct that identifies the stream we're currently working with.
    /// # Remarks
    /// * Requires `self.src_pads` to be exclusively locked for writing. Please ensure that it is
    /// unlocked when calling this function.
    fn push_stream_start_on_all_pads(&self, stream_identifier: &StreamIdentifier) {
        gst_debug!(CAT, "Pushing stream start event for all streams");
        for (stream_name, src_pad) in self.src_pads.write().unwrap().iter_mut() {
            Self::push_stream_start(src_pad, stream_name, stream_identifier);
        }
    }

    /// Reset the internals of `rgbddemux`, so that they can be used again after re-negotiation.
    fn reset(&self) {
        // Reset internals (except for settings)
        self.flow_combiner.lock().unwrap().reset();
        *self.stream_identifier.lock().unwrap() = None;
        // Reset stream start tracker, but keep the pads
        for (_, src_pad) in self.src_pads.write().unwrap().iter_mut() {
            src_pad.pushed_stream_start = false;
        }
    }
}

impl GstObjectImpl for RgbdDemux {}
impl ObjectImpl for RgbdDemux {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<[glib::ParamSpec; 1]> = Lazy::new(|| {
            [ParamSpecBoolean::new(
                "distribute-timestamps",
                "Distribute Timestamps",
                "If enabled, timestamps of the main buffers will be distributed to the
                 auxiliary buffers embedded within the `video/rbgd` stream.",
                DEFAULT_DISTRIBUTE_TIMESTAMPS,
                ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn constructed(&self, element: &Self::Type) {
        self.parent_constructed(element);

        element
            .add_pad(&self.sink_pad)
            .expect("rgbddemux: Failed to add sink pad to the element");
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        let mut settings = self.settings.write().unwrap();

        match pspec.name() {
            prop @ "distribute-timestamps" => {
                let distribute_timestamps =
                    get_property_and_debug(*CAT, value, prop, settings.distribute_timestamps);
                settings.distribute_timestamps = distribute_timestamps;
            }
            _ => unimplemented!("Property is not implemented"),
        };
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        let settings = self.settings.read().unwrap();

        match pspec.name() {
            "distribute-timestamps" => settings.distribute_timestamps.to_value(),
            _ => unimplemented!("Property is not implemented"),
        }
    }
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "rgbddemux",
        gst::Rank::None,
        RgbdDemux::type_(),
    )
}
