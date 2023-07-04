use crate::error::Error;
use crate::pipeline_profile::PipelineProfile;
use crate::stream_profile::{StreamData, StreamResolution};

/// Helper struct for `get_info_all_streams()`.
pub struct StreamInfo {
    pub data: StreamData,
    pub resolution: StreamResolution,
}

/// Retrieve information about all enabled streams based on a running
/// [`Pipeline`](../pipeline/struct.Pipeline.html).
///
/// # Arguments
/// * pipeline_profile - The [`PipelineProfile`](../pipeline_profile/struct.PipelineProfile.html)
/// to extract the information from.
///
/// # Returns
/// * `Ok(Vec<StreamInfo>)` on success.
/// * `Err(Error)` on failure.
pub fn get_info_all_streams(pipeline_profile: &PipelineProfile) -> Result<Vec<StreamInfo>, Error> {
    let mut info_all_streams: Vec<StreamInfo> = Vec::new();
    let streams = pipeline_profile.get_streams()?;
    for i in 0..streams.count()? {
        let stream_profile = streams.get(i)?;
        let stream_data = stream_profile.get_data()?;
        let stream_resolution = stream_profile.get_resolution()?;
        info_all_streams.push(StreamInfo {
            data: stream_data,
            resolution: stream_resolution,
        })
    }
    Ok(info_all_streams)
}
