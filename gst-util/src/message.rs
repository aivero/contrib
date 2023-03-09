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
