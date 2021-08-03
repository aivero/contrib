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

use glib::translate::from_glib;
use gst::meta::*;
use gst::BufferRef;
use std::fmt;

use crate::rgbd::sys;
pub use crate::rgbd::sys::TagsMeta;

/// The TagsMeta API is intended to allow developers to add Tags onto gst buffers, which can be used
/// to identify different buffers from each other.
impl TagsMeta {
    /// Adds a TagList as metadata onto the given `buffer`.
    /// # Arguments
    /// * `buffer` - The buffer onto which the TagList should be added.
    /// * `meta_tags` - The TagsList that should be added as metadata.
    /// # Example
    /// ```
    /// use gstreamer_depth_meta::tags::TagsMeta;
    /// gstreamer::init().unwrap();
    /// let mut main_buffer = gstreamer::Buffer::new();
    /// let mut tags = gstreamer::tags::TagList::new();
    /// tags.get_mut().unwrap()
    ///     .add::<gstreamer::tags::Title>(&"example-tag", gstreamer::TagMergeMode::Append);
    /// TagsMeta::add(
    ///    main_buffer.make_mut(),
    ///    &mut tags,
    /// );
    /// ```
    pub fn add<'a>(
        buffer: &'a mut BufferRef,
        meta_tags: &mut gst::TagList,
    ) -> MetaRefMut<'a, Self, Standalone> {
        unsafe {
            let meta = sys::tags_meta_add(buffer.as_mut_ptr(), meta_tags.as_mut_ptr());
            Self::from_mut_ptr(buffer, meta)
        }
    }

    /// Gets the first [TagsMeta](struct.TagsMeta.html) attached onto the given `buffer`.
    /// # Arguments
    /// * `buffer` - A reference to the buffer, from which the [TagsMeta](struct.TagsMeta.html) should be read.
    pub fn get(buffer: &BufferRef) -> &Self {
        unsafe { &*sys::tags_meta_get(buffer.as_mut_ptr()) }
    }

    /// Get the `gst::TagList` associated with the TagsMeta.
    /// # Returns
    /// A list of tags on the buffer.
    pub fn get_tag_list(&self) -> gst::TagList {
        unsafe { gst::tags::TagList::from_glib_none(self.tags) }
    }
}

unsafe impl MetaAPI for TagsMeta {
    type GstType = TagsMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(sys::tags_meta_api_get_type()) }
    }
}

impl fmt::Debug for TagsMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TagsMeta")
            .field("tags", &self.tags)
            .finish()
    }
}

pub fn get_tags(tag: &str) -> gst::TagList {
    let mut tags = gst::tags::TagList::new();
    tags.get_mut()
        .unwrap()
        .add::<gst::tags::Title>(&tag, gst::TagMergeMode::Append);
    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_expect_tags_equal() {
        // Arrange
        gst::init().unwrap();
        let input_title = "example-tag";
        let mut buffer = gst::Buffer::new();
        let mut tags = get_tags(input_title);

        // Act
        TagsMeta::add(buffer.make_mut(), &mut tags);

        // Assert
        // convert get the Title tag from the MetaAPI
        let meta = TagsMeta::get(buffer.get_mut().unwrap());
        let tag_list = unsafe { gst::tags::TagList::from_glib_none(meta.tags) };

        // Get the tag title from GstTagList
        let gst_tag_title = &tag_list.get::<gst::tags::Title>().unwrap();
        let title = gst_tag_title.get().unwrap();

        assert_eq!(input_title, title);
    }

    #[test]
    fn add_and_remove_expect_no_tags_present() {
        // Arrange
        gst::init().unwrap();
        let input_title = "example-tag";
        let mut buffer = gst::Buffer::new();
        let mut tags = get_tags(input_title);
        let tags_meta = TagsMeta::add(buffer.make_mut(), &mut tags);

        // Act
        tags_meta.remove();

        // Assert
        for i in buffer.iter_meta::<TagsMeta>() {
            assert_eq!(
                true, false,
                "A TagsMeta was still present on the buffer: {:#?}",
                i
            )
        }
    }
}
