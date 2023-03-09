/// An enum that countains source of the timestamp, either Image or ImuSample.
pub(crate) enum TimestampSource<'a> {
    Image(&'a libk4a::image::Image),
    ImuSample(&'a libk4a::imu_sample::ImuSample),
}

impl<'a> TimestampSource<'a> {
    /// Extract timestamp either from `Image` or `ImuSample`
    ///
    /// # Returns
    /// * `gst::ClockTime` containing the timestamp.
    pub(crate) fn extract_timestamp(&self) -> gst::ClockTime {
        match self {
            TimestampSource::Image(image) => gst::ClockTime::from_useconds(image.get_timestamp()),
            TimestampSource::ImuSample(imu_sample) => {
                gst::ClockTime::from_useconds(imu_sample.get_acc_timestamp())
            }
        }
    }
}
