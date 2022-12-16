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

use gst::glib;
use gst::{subclass::prelude::*, Event};
use gst_base::prelude::*;
use gst_base::subclass::prelude::*;
use gst_base::AggregatorPad;
use gst_depth_meta::buffer::BufferMeta;
use gst_depth_meta::rgbd;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "rgbdmux",
        gst::DebugColorFlags::empty(),
        Some("RGB-D Muxer"),
    )
});

struct State {
    /// The current framerate out the rgbd output
    framerate: Option<gst::Fraction>,
    /// Whether we are outputting our first buffer (after flush)
    first: bool,
}

/// A struct representation of the `rgbdmux` element.
pub struct RgbdMux {
    state: Mutex<State>,
}

glib::wrapper! {
    pub struct RgbdMuxObject(ObjectSubclass<RgbdMux>)
        @extends gst_base::Aggregator, gst::Element, gst::Object;
}

#[glib::object_subclass]
impl ObjectSubclass for RgbdMux {
    const NAME: &'static str = "rgbdmux";
    type Type = RgbdMuxObject;
    type ParentType = gst_base::Aggregator;

    fn new() -> Self {
        Self {
            state: Mutex::new(State {
                framerate: None,
                first: true,
            }),
        }
    }
}

impl AggregatorImpl for RgbdMux {
    /// Called whenever a event is received at one of the sink pads.
    /// # Arguments
    /// * `aggregator` - The element that represents the `rgbdmux` in GStreamer.
    /// * `aggregator_pad` - The pad that received the event.
    /// * `event` - The event that should be handled.
    fn sink_event(
        &self,
        aggregator: &RgbdMuxObject,
        aggregator_pad: &AggregatorPad,
        event: Event,
    ) -> bool {
        match event.view() {
            gst::EventView::Tag(_) => {
                let src_pad = aggregator.static_pad("src").unwrap();
                if !src_pad.push_event(event) {
                    gst_warning!(CAT, "Could not send tag event");
                }
                true
            }
            gst::EventView::Caps(_) => {
                // Sink caps changed. Mark src pad as needing to be reconfigured so that
                // `update_src_caps` gets called again.
                let src_pad = aggregator.static_pad("src").unwrap();
                src_pad.mark_reconfigure();

                self.parent_sink_event(aggregator, aggregator_pad, event)
            }
            _ => self.parent_sink_event(aggregator, aggregator_pad, event),
        }
    }

    /// The default implementation parses `sink_%d` no matter what specified in the template.
    /// We request pads with `sink_%s` such as sink_depth. The default impl produces a `sink_0` pad from that.
    fn create_new_pad(
        &self,
        _aggregator: &Self::Type,
        templ: &gst::PadTemplate,
        req_name: Option<&str>,
        _caps: Option<&gst::Caps>,
    ) -> Option<AggregatorPad> {
        let req_name = req_name?;
        if req_name.starts_with("sink_") {
            gst::Pad::from_template(templ, Some(req_name))
                .downcast()
                .ok()
        } else {
            None
        }
    }

    fn flush(&self, _aggregator: &RgbdMuxObject) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.state.lock().unwrap().first = true;
        Ok(gst::FlowSuccess::Ok)
    }

    /// Called when buffers are queued on all sinkpads. Classes should iterate the GstElement->sinkpads and peek or steal
    /// buffers from the GstAggregatorPad.
    /// # Arguments
    /// * `aggregator` - The element that represents the `rgbdmux` in GStreamer.
    /// * `timeout` - represents if this is the last chance to produce data given the configured framerate
    fn aggregate(
        &self,
        aggregator: &RgbdMuxObject,
        timeout: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let src_pads = aggregator.src_pads();
        let sink_pads = aggregator.sink_pads();
        let mut sink_pads = sink_pads
            .iter()
            .filter_map(|sp| sp.downcast_ref::<gst_base::AggregatorPad>());
        let agg_pad = src_pads[0]
            .downcast_ref::<gst_base::AggregatorPad>()
            .unwrap();

        let any_pad_is_eos = sink_pads
            .clone()
            .any(|pad| pad.is_eos() && !pad.has_buffer());

        if any_pad_is_eos {
            gst_debug!(CAT, "Got EOS. We are done");
            return Err(gst::FlowError::Eos);
        }
        if agg_pad.caps().is_none() {
            gst_debug!(CAT, "We don't know caps yet");
            return Err(gst_base::AGGREGATOR_FLOW_NEED_DATA);
        }

        // Get the current deadline time or desired output time
        let segment: gst::FormattedSegment<gst::ClockTime> = agg_pad.segment().downcast().unwrap();
        let segment_position = segment.position().or_else(|| segment.start());
        let position_running_time = segment.to_running_time(segment_position).unwrap();

        let (duration, first) = {
            let state = self.state.lock().unwrap();
            (
                Self::get_duration_from_fps(state.framerate.unwrap()).unwrap(),
                state.first,
            )
        };

        gst_debug!(
            CAT,
            "Segment position: {:?}, start: {:?}, end: {:?}",
            segment.position(),
            segment.start(),
            segment.stop()
        );

        let mut has_all_buffers_in_range = true;
        for sink_pad in sink_pads.clone() {
            let buffer_in_range = self.drop_out_of_range_buffers_on_pad(
                aggregator,
                position_running_time,
                duration,
                sink_pad,
                first,
            )?;
            has_all_buffers_in_range = has_all_buffers_in_range && buffer_in_range;
        }

        if timeout {
            let basetime = aggregator.base_time();

            // HACK: In certain pipelines, aggregate gets called before the transition from
            // PausedToPlaying. In that case, the basetime has not yet been set to something
            // useful for the aggregator. This causes all kinds of problems, and we are not
            // sure of the right solution, so for now we just ignore basetimes of zero.
            if basetime.unwrap_or(gst::ClockTime::ZERO) == gst::ClockTime::ZERO {
                return Ok(gst::FlowSuccess::Ok);
            }

            gst_debug!(CAT, "Timeout. Sending Gap event");
            self.send_gap_event(aggregator).map_err(|_| {
                self.advance_segment_position(aggregator);
                gst::FlowError::Error
            })?;
            self.advance_segment_position(aggregator);
            return Ok(gst::FlowSuccess::Ok);
        }

        let all_pads_have_buffers = sink_pads.all(|pad| pad.has_buffer());
        if all_pads_have_buffers && !has_all_buffers_in_range {
            gst_debug!(
                CAT,
                "Queues are full, but not all buffers are in range. Sending Gap event",
            );

            self.send_gap_event(aggregator).map_err(|_| {
                self.advance_segment_position(aggregator);
                gst::FlowError::Error
            })?;
            self.advance_segment_position(aggregator);
            return Ok(gst::FlowSuccess::Ok);
        }

        if has_all_buffers_in_range {
            gst_debug!(CAT, "All buffers in range, muxing");

            self.advance_segment_position(aggregator);

            // https://gstreamer.freedesktop.org/documentation/base/gstaggregator.html?gi-language=c#gst_aggregator_selected_samples
            aggregator.selected_samples(
                position_running_time,
                position_running_time,
                duration,
                None,
            );
            let outbuf = self
                .mux_buffers_set_ts(aggregator, segment_position, Some(duration))
                .map_err(|_| gst::FlowError::Error)?;

            gst_debug!(CAT, "Muxed buffers, finishing");

            self.finish_buffer(aggregator, outbuf)?;

            self.state.lock().unwrap().first = false;

            Ok(gst::FlowSuccess::Ok)
        } else {
            gst_debug!(CAT, "Need more data");
            Err(gst_base::AGGREGATOR_FLOW_NEED_DATA)
        }
    }

    /// This function is called during CAPS negotiation. It can be used to decide on a CAPS format
    /// or delay the negotiation until sufficient data is present to decide on the CAPS (in this
    /// case when an upstream element has requested sink pads)
    /// # Arguments
    /// * `aggregator` - A reference to the element that represents `rgbdmux` in GStreamer.
    /// * `_caps` - (not used) The CAPS that is currently negotiated for the element.
    fn update_src_caps(
        &self,
        aggregator: &RgbdMuxObject,
        _caps: &gst::Caps,
    ) -> Result<gst::Caps, gst::FlowError> {
        // Join all the pad names to create the 'streams' section of the CAPS
        let sink_pads = aggregator.sink_pads();
        let sink_pads = sink_pads
            .iter()
            .filter_map(|pad| pad.downcast_ref::<gst_base::AggregatorPad>());

        // Find the lowest framerate defined across all the caps of our sinkpads
        // The framerate is the lowest framerate found across all sinkpads,
        // since we will drop all frames if one frame is missing
        let min_framerate = sink_pads
            .clone()
            .filter_map(|pad| {
                let caps = pad.current_caps()?;
                let structure = caps.structure(0).unwrap();
                structure.get::<gst::Fraction>("framerate").ok()
            })
            .min();
        self.state.lock().unwrap().framerate = min_framerate;

        if min_framerate.is_none() {
            gst_debug!(CAT, "None of the sink pads had a framerate");
            return Err(gst_base::AGGREGATOR_FLOW_NEED_DATA);
        }

        if sink_pads.clone().any(|pad| pad.current_caps().is_none()) {
            gst_debug!(CAT, "No sink pads yet");
            return Err(gst_base::AGGREGATOR_FLOW_NEED_DATA);
        }

        let sink_pad_caps = sink_pads.clone().map(|pad| pad.current_caps().unwrap());
        let sink_pad_names = sink_pads.clone().map(|pad| pad.name());

        let streams = sink_pad_names
            .clone()
            .map(|s| s.trim_start_matches("sink_").to_send_value());

        let mut downstream_caps = gst::Caps::new_simple(
            "video/rgbd",
            &[
                ("streams", &gst::Array::from_values(streams)),
                ("framerate", &min_framerate.unwrap()),
            ],
        );

        let mut_caps = downstream_caps.make_mut().structure_mut(0).unwrap();

        // Map the caps into their corresponding stream formats
        for (caps, pad_name) in sink_pad_caps.zip(sink_pad_names) {
            self.compose_elementary_caps_to_rgbd_caps(&caps, &pad_name, mut_caps)
        }

        Ok(downstream_caps.to_owned())
    }

    /// Called when the element needs to know the running time of the next rendered buffer for live pipelines.
    /// Returns the aggregator.simple_get_next_time() unless drop_if_missing is set to false
    /// This causes deadline based aggregation to occur. Returning GST_ClockTime::NONE causes the element to
    /// wait for buffers on all sink pads before aggregating.
    /// # Arguments
    /// * `aggregator` - A reference to the element that represents `rgbdmux` in GStreamer.
    fn next_time(&self, aggregator: &RgbdMuxObject) -> Option<gst::ClockTime> {
        let nt = aggregator.simple_get_next_time();
        gst_debug!(CAT, "Aggregator next_time: {:?}", nt);
        nt
    }

    /// Called when the src caps have been negotiated
    /// We are setting the latency here.
    fn negotiated_src_caps(
        &self,
        aggregator: &Self::Type,
        caps: &gst::Caps,
    ) -> Result<(), gst::LoggableError> {
        // Framerate should be known at this point in time
        let framerate = self.state.lock().unwrap().framerate.unwrap();
        let duration = RgbdMux::get_duration_from_fps(framerate).unwrap();

        gst_debug!(CAT, "Setting latency to {}ms", duration.mseconds());
        aggregator.set_latency(duration, duration);

        gst_debug!(CAT, "Negotiated src caps are: {}", caps);
        self.parent_negotiated_src_caps(aggregator, caps)
    }

    /// Called when the element goes from PAUSED to READY.
    /// # Arguments
    /// * `aggregator` - The element that represents the `rgbdmux` in GStreamer.
    fn stop(&self, aggregator: &RgbdMuxObject) -> Result<(), gst::ErrorMessage> {
        // Reset internals (except for settings)
        self.parent_stop(aggregator)
    }
}

impl ElementImpl for RgbdMux {
    /// This function provides a custom implementation to what should happen when request pads are
    /// released.
    /// # Arguments
    /// * `element` - The element that represents `rgbdmux` in GStreamer.
    /// * `pad` - The pad that is soon to be released.
    fn release_pad(&self, element: &Self::Type, pad: &gst::Pad) {
        // De-activate the pad
        pad.set_active(false)
            .unwrap_or_else(|_| panic!("Could not deactivate a sink pad: {:?}", pad));

        // Remove the pad from the element
        element
            .remove_pad(pad)
            .unwrap_or_else(|_| panic!("Could not remove a sink pad: {:?}", pad));

        // Remove the pad from our internal reference HashMap
        let pad_name = pad.name().as_str().to_string();
        gst_debug!(CAT, obj: element, "release_pad: {}", pad_name);

        // Mark src pad for reconfiguration and let the base class renegotiate right before the next call to aggregate()
        let src_pad = element
            .static_pad("src")
            .expect("rgbdmux: Subclass of GstAggregator must have a src pad");
        src_pad.mark_reconfigure();
    }

    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "RGB-D Muxer",
                "Muxer/RGB-D",
                "Muxes multiple elementary streams into a single `video/rgbd` stream",
                "Andrej Orsula <andrej.orsula@aivero.com>, \
                 Tobias Morell <tobias.morell@aivero.com>, \
                 Raphael Duerscheid <raphael.duerscheid@aivero.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }

    fn pad_templates() -> &'static [gst::PadTemplate] {
        static PAD_TEMPLATES: Lazy<[gst::PadTemplate; 2]> = Lazy::new(|| {
            let mut sink_caps = gst::Caps::new_simple("video/x-raw", &[]);
            {
                let sink_caps = sink_caps.get_mut().unwrap();
                sink_caps.append(gst::Caps::new_simple("meta/x-klv", &[("parsed", &true)]));
                sink_caps.append(gst::Caps::new_simple("image/jpeg", &[]));
            }

            [
                gst::PadTemplate::with_gtype(
                    "sink_%s",
                    gst::PadDirection::Sink,
                    gst::PadPresence::Request,
                    &sink_caps,
                    gst_base::AggregatorPad::static_type(),
                )
                .expect("rgbdmux: Failed to add 'sink_%s' pad template"),
                gst::PadTemplate::with_gtype(
                    "src",
                    gst::PadDirection::Src,
                    gst::PadPresence::Always,
                    &gst::Caps::new_simple("video/rgbd", &[]),
                    gst_base::AggregatorPad::static_type(),
                )
                .expect("rgbdmux: Failed to add 'src' pad template"),
            ]
        });

        PAD_TEMPLATES.as_ref()
    }
}

impl RgbdMux {
    /// Advances the segment position of an gst_base::Aggregator's srcpad.segment
    /// This is a requirement for aggregator subclasses that use the `aggregator.simple_get_next_time()`
    /// for the required trait impl function `next_time()`
    /// # Arguments
    /// * `aggregator` - The aggregator to consider.
    fn advance_segment_position(&self, aggregator: &RgbdMuxObject) {
        let src_pads = aggregator.src_pads();
        let agg_pad = src_pads[0]
            .downcast_ref::<gst_base::AggregatorPad>()
            .unwrap();

        let mut segment: gst::FormattedSegment<gst::ClockTime> =
            agg_pad.segment().downcast().unwrap();

        let start: gst::ClockTime = segment.start().unwrap_or(gst::ClockTime::ZERO);
        let end: gst::ClockTime = segment.stop().unwrap_or(gst::ClockTime::MAX);
        let pts: gst::ClockTime = segment.position().unwrap_or(start);

        let framerate = self.state.lock().unwrap().framerate.unwrap();
        let duration = Self::get_duration_from_fps(framerate);
        let new_position = if segment.rate() > 0.0 {
            pts + duration.unwrap()
        } else {
            pts - duration.unwrap()
        };

        let new_position = new_position.clamp(start, end);
        segment.set_position(new_position);
        gst_debug!(CAT, "Advanced segment to {}", new_position.display());

        // https://gstreamer.freedesktop.org/documentation/base/gstaggregator.html?gi-language=c#gst_aggregator_update_segment
        aggregator.update_segment(&segment);
    }

    /// Converts a framerate specified as a fraction in seconds to a gst::ClockTime
    /// # Arguments
    /// * `framerate` - fraction specified in seconds 30fps -> 30/1
    /// # returns
    /// * Some(duration) - if could convert the fraction to its duration
    /// * None - if the numerator was zero
    fn get_duration_from_fps(framerate: gst::Fraction) -> Option<gst::ClockTime> {
        gst::ClockTime::SECOND.mul_div_floor(framerate.denom() as u64, framerate.numer() as u64)
    }

    /// Pops / removes all buffers from a given pad that are outside of the range
    /// span by `position_running_time ... (position_running_time + duration)
    /// # Arguments
    /// * position_running_time - The current position defined by the aggregator's src_pad.segment
    /// * duration - The inverse of the negotiated framerate for the aggregator's src caps
    /// * sink_pad - The pad on which to check and pop buffers
    fn drop_out_of_range_buffers_on_pad(
        &self,
        aggregator: &RgbdMuxObject,
        position_running_time: gst::ClockTime,
        duration: gst::ClockTime,
        sink_pad: &gst_base::AggregatorPad,
        first: bool,
    ) -> Result<bool, gst::FlowError> {
        while let Some(buffer) = sink_pad.peek_buffer() {
            let segment = sink_pad.segment().downcast::<gst::ClockTime>().unwrap();
            let pts = buffer.pts().ok_or(gst::FlowError::Error)?;
            let buffer_duration = buffer.duration().unwrap_or_default();
            let latency = aggregator.latency().unwrap_or_default();
            let buffer_running_time =
                segment.to_running_time(pts).unwrap_or(gst::ClockTime::ZERO) + latency;

            // Buffers TS are in range if they are equal to the current deadline or in the future,
            // but not more than `duration` in the future.
            //  if(buffer_running_time >= *position_running_time
            //     && buffer_running_time < position_running_time + duration){
            //     valid_buffer
            // }

            if buffer_running_time + buffer_duration < position_running_time {
                sink_pad.drop_buffer();
                gst_info!(
                    CAT,
                    "Dropped buffer: {} {} {} {}",
                    sink_pad.name(),
                    buffer_running_time,
                    position_running_time,
                    latency,
                );
            } else if first || buffer_running_time < position_running_time + duration {
                return Ok(true);
            } else {
                gst_debug!(
                    CAT,
                    "Early buffer: {} {} {}",
                    sink_pad.name(),
                    buffer_running_time,
                    position_running_time
                );
                return Ok(false);
            }
        }
        Ok(false)
    }
    /// Mux all buffers to a single output buffer. All buffers are properly tagget with a title.
    /// # Arguments
    /// * `aggregator` - The aggregator to consider.
    /// # Returns:
    /// * OK(buf) - The main buffer, containing aux buffers as BufferMeta
    /// * Err
    fn mux_buffers_set_ts(
        &self,
        aggregator: &RgbdMuxObject,
        ts: Option<gst::ClockTime>,
        duration: Option<gst::ClockTime>,
    ) -> Result<gst::Buffer, gst::ErrorMessage> {
        let sink_pads = aggregator.sink_pads();
        let mut sink_pads = sink_pads
            .iter()
            .filter_map(|sp| sp.downcast_ref::<gst_base::AggregatorPad>());

        // Use the first buffer in the aggregator.sink_pads as the buffer we send out
        let first_pad = sink_pads.next().unwrap();
        let first_pad_name = first_pad.name().to_string();

        let mut main_buffer = match first_pad.pop_buffer() {
            // We have a buffer, let's tag it
            Some(mut buf) => {
                let bref = buf.make_mut();
                bref.set_dts(ts);
                bref.set_pts(ts);
                bref.set_duration(duration);
                let stream_name = first_pad_name.trim_start_matches("sink_");
                rgbd::tag_buffer_with_title(bref, stream_name)?;
                buf
            }
            // There is no buffer, let's send a gap event
            None => {
                return Err(gst::error_msg!(gst::CoreError::Pad, ["No buffer found"]));
            }
        };

        // Iterate over all other sink pads, excluding the first one (already processed)
        // For each pad, get a tagged buffer and attach it to the main buffer
        // If a sink pad has no buffer queued, create an empty GAP buffer and attach it to the main buffer as well
        for agg_pad in sink_pads {
            self.attach_aux_buffers(agg_pad, main_buffer.make_mut(), ts, duration)?;
        }

        gst_debug!(CAT, obj: aggregator, "A frameset was muxed.");
        Ok(main_buffer)
    }

    /// Get a tagged buffer from pad `sink_pad_name` and attach it to `main_buffer`. If a sink pad has no buffer queued,
    /// create an empty GAP buffer and attach it to the main buffer as well.
    /// # Arguments
    /// * `aggregator` - The aggregator to consider.
    /// * `sink_pad_names` - The vector containing all sink pad names.
    /// * `main_buffer` - Mutable reference to the main buffer to which we attach all auxiliary buffers.
    fn attach_aux_buffers(
        &self,
        sink_pad: &AggregatorPad,
        main_buffer: &mut gst::BufferRef,
        ts: Option<gst::ClockTime>,
        duration: Option<gst::ClockTime>,
    ) -> Result<(), gst::ErrorMessage> {
        let stream_name = sink_pad.name();
        let stream_name = stream_name.as_str().trim_start_matches("sink_");
        match sink_pad.pop_buffer() {
            Some(mut buffer) => {
                let bufref = buffer.make_mut();
                bufref.set_dts(ts);
                bufref.set_pts(ts);
                bufref.set_duration(duration);
                rgbd::tag_buffer_with_title(bufref, stream_name)?;
                BufferMeta::add(main_buffer, &mut buffer);
                Ok(())
            }
            None => Err(gst::error_msg!(
                gst::CoreError::Pad,
                ["No buffer is queued on auxiliary `{}` pad.", stream_name]
            )),
        }
    }

    /// Sends a gap event downstream.
    /// # Arguments
    /// * `aggregator` - The aggregator to drop all queued buffers for.
    fn send_gap_event(&self, aggregator: &RgbdMuxObject) -> Result<(), gst::ErrorMessage> {
        gst_debug!(CAT, "Sending gap event");
        // Get the current position aka deadline as
        let src_pads = aggregator.src_pads();
        let agg_pad: &gst_base::AggregatorPad = src_pads[0].downcast_ref().unwrap();

        let segment: gst::FormattedSegment<gst::ClockTime> = agg_pad.segment().downcast().unwrap();
        let pts: gst::ClockTime = segment
            .position()
            .unwrap_or_else(|| segment.start().unwrap());
        //todo: Unclear if we have to use the running_time to create the gap event
        // let running_time = segment.to_running_time(pts).unwrap();

        let framerate = self.state.lock().unwrap().framerate.unwrap();
        let duration = RgbdMux::get_duration_from_fps(framerate);

        // Create a GAP event with duration
        let gap_event = gst::event::Gap::new(pts, duration);

        // And send it downstream
        if agg_pad.push_event(gap_event) {
            gst_debug!(CAT, obj: aggregator, "Sending of GAP event was successful");
        } else {
            // This might have been caused by a Flushing return, that
            // information is lost by the conversion to boolean. The usual
            // workaround is to ignore the error, and let the error happen further
            // on on an actual pad push()
            gst_warning!(CAT, obj: aggregator, "Failed to send gap event");
        }

        Ok(())
    }

    /// Extracts the relevant fields from the pad's CAPS and converts them into a tuple containing
    /// the field's name as the first and its value as second.
    /// # Arguments
    /// * `pad_caps` - A reference to the pad's CAPS.
    /// * `pad_name` - The name of the stream we're currently generating CAPS for.
    fn compose_elementary_caps_to_rgbd_caps(
        &self,
        pad_caps: &gst::Caps,
        pad_name: &str,
        src_caps: &mut gst::StructureRef,
    ) {
        let stream_name = pad_name.trim_start_matches("sink_");
        // Set the format for MJPG stream
        if pad_caps.is_subset(gst::Caps::new_simple("image/jpeg", &[]).as_ref()) {
            let src_field_name = format!("{}_format", stream_name);
            src_caps.set(&src_field_name, &"image/jpeg");
        }

        // Filter out all CAPS we don't care about and map those we do into strings
        let pad_caps = pad_caps.iter().next().expect("rgbdmux: Got empty CAPS");
        if let Ok(format) = pad_caps.value("format") {
            src_caps.set_value(&format!("{}_format", stream_name), format.clone());
        }
        if let Ok(width) = pad_caps.value("width") {
            src_caps.set_value(&format!("{}_width", stream_name), width.clone());
        }
        if let Ok(height) = pad_caps.value("height") {
            src_caps.set_value(&format!("{}_height", stream_name), height.clone());
        }
    }
}

impl GstObjectImpl for RgbdMux {}
impl ObjectImpl for RgbdMux {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        // obj.set_start_time_selection(gst_base::AggregatorStartTimeSelection::First);
    }
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(Some(plugin), "rgbdmux", gst::Rank::None, RgbdMux::type_())
}
