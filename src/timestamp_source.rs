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

/// An enum that countains source of the timestamp, either Image or ImuSample.
pub(crate) enum TimestampSource<'a> {
    Image(&'a k4a::image::Image),
    ImuSample(&'a k4a::imu_sample::ImuSample),
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
