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

use gst::prelude::*;

pub trait MessageExtension {
    /// Dump a dot file if the message is deemed important.
    fn dump_dot_if_important(&self, bin: &gst::Bin);
}

impl MessageExtension for gst::MessageRef {
    fn dump_dot_if_important(&self, bin: &gst::Bin) {
        let src_name = self
            .src()
            .map(|s| s.name())
            .unwrap_or_else(|| "unknown".into());

        match self.view() {
            gst::MessageView::StateChanged(state) => gst::debug_bin_to_dot_file_with_ts(
                bin,
                gst::DebugGraphDetails::all(),
                &format!(
                    "{}-{}-{:?}-{:?}-{:?}",
                    bin.name(),
                    src_name,
                    state.old(),
                    state.current(),
                    state.pending()
                ),
            ),
            gst::MessageView::Eos(_) => gst::debug_bin_to_dot_file_with_ts(
                bin,
                gst::DebugGraphDetails::all(),
                &format!("{}-{}-eos", bin.name(), src_name),
            ),
            gst::MessageView::Error(_) => gst::debug_bin_to_dot_file_with_ts(
                bin,
                gst::DebugGraphDetails::all(),
                &format!("{}-{}-error", bin.name(), src_name),
            ),
            gst::MessageView::Warning(_) => gst::debug_bin_to_dot_file_with_ts(
                bin,
                gst::DebugGraphDetails::all(),
                &format!("{}-{}-warning", bin.name(), src_name),
            ),
            _ => {}
        }
    }
}
