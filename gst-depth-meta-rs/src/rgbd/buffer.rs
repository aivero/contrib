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
use gst::glib;
use gst::meta::*;
use gst::BufferRef;
use std::fmt;

use crate::rgbd::sys;
pub use crate::rgbd::sys::BufferMeta;

/// The BufferMeta API is intended to allow developers to add gst buffers as metadata onto buffers.
/// This represents a very generic container format that can hold basically any data.
impl BufferMeta {
    /// Add the given `meta_buffer` onto the given `buffer`, effectively stacking `meta_buffer` onto
    /// `buffer`.
    /// # Arguments
    /// * `buffer` - The base buffer, to which the metadata should be added.
    /// * `meta_buffer` - The buffer that should be added as metadata.
    /// # Example
    /// ```
    /// use gst_depth_meta::buffer::BufferMeta;
    /// gst::init().unwrap();
    /// let mut main_buffer = gst::Buffer::new();
    /// let mut meta_buffer = gst::Buffer::from_slice([10,9,8,7]);
    /// BufferMeta::add(
    ///    main_buffer.make_mut(),
    ///    &mut meta_buffer,
    /// );
    /// ```
    pub fn add<'a>(
        buffer: &'a mut BufferRef,
        meta_buffer: &mut gst::Buffer,
    ) -> MetaRefMut<'a, Self, Standalone> {
        unsafe {
            let meta = sys::buffer_meta_add(buffer.as_mut_ptr(), meta_buffer.as_mut_ptr());
            Self::from_mut_ptr(buffer, meta)
        }
    }

    /// Get a `BufferMeta` on the given buffer.
    /// # Arguments
    /// * `buffer` - The buffer to get a meta buffer from.
    pub fn get(buffer: &BufferRef) -> &BufferMeta {
        unsafe { &*sys::buffer_meta_get(buffer.as_mut_ptr()) }
    }

    /// Get the contained buffer
    pub fn buffer(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr(self.buffer) }
    }

    /// Get a mutable reference to the contained buffer, if possible, or make it mutable first
    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        unsafe {
            if gst_sys::gst_mini_object_is_writable(self.buffer as *mut _) == glib_sys::GFALSE {
                self.buffer =
                    gst_sys::gst_mini_object_make_writable(self.buffer as *mut _) as *mut _;
            }
            gst::BufferRef::from_mut_ptr(self.buffer)
        }
    }

    /// Get the contained buffer
    pub fn buffer_owned(&self) -> gst::Buffer {
        unsafe { gst::Buffer::from_glib_none(self.buffer) }
    }
}

unsafe impl MetaAPI for BufferMeta {
    type GstType = BufferMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(sys::buffer_meta_api_get_type()) }
    }
}

impl fmt::Debug for BufferMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BufferMeta")
            .field("buffer", &self.buffer())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_expect_buffers_equal() {
        // Arrange
        gst::init().unwrap();
        let mut buffer = gst::Buffer::new();
        let input = [9, 8, 7, 6];
        let mut meta_buffer = gst::Buffer::from_slice(input);
        let buffer_mut = buffer.make_mut();

        // Act
        BufferMeta::add(buffer_mut, &mut meta_buffer);

        // Assert
        // convert from MetaBuffer into a [u8]
        let meta = BufferMeta::get(buffer_mut);
        let meta_buffer_out = unsafe { gst::buffer::Buffer::from_glib_none(meta.buffer) };
        let meta_buffer_out = meta_buffer_out.map_readable().unwrap();
        let output = meta_buffer_out.as_slice();

        for i in 0..input.len() {
            assert_eq!(input[i], output[i], "Failed for index {}", i);
        }
    }

    #[test]
    fn add_two_and_get_expect_both_buffers_present() {
        // Arrange
        gst::init().unwrap();
        let mut buffer = gst::Buffer::new();
        let input = [9, 8, 7, 6];
        let input_2 = [5, 4, 3, 2];
        let mut meta_buffer = gst::Buffer::from_slice(input);
        let mut meta_buffer_2 = gst::Buffer::from_slice(input_2);
        let buffer_mut = buffer.make_mut();

        // Act
        BufferMeta::add(buffer_mut, &mut meta_buffer);
        BufferMeta::add(buffer_mut, &mut meta_buffer_2);

        let mut number_of_buffers = 0;
        // Assert
        for meta in buffer.iter_meta::<BufferMeta>() {
            let meta_buffer_out = unsafe { gst::buffer::Buffer::from_glib_none(meta.buffer) };
            let meta_buffer_out = meta_buffer_out.map_readable().unwrap();
            let output = meta_buffer_out.as_slice();

            if number_of_buffers == 0 {
                for i in 0..input.len() {
                    assert_eq!(input[i], output[i], "Failed for index {}", i);
                }
            } else {
                for i in 0..input_2.len() {
                    assert_eq!(input_2[i], output[i], "Failed for index {}", i);
                }
            }
            number_of_buffers += 1;
        }
        assert_eq!(number_of_buffers, 2, "Found a wrong number of buffers");
    }

    #[test]
    fn add_and_remove_expect_no_buffers_present() {
        // Arrange
        gst::init().unwrap();
        let mut buffer = gst::Buffer::new();
        let input = [9, 8, 7, 6];
        let mut meta_buffer = gst::Buffer::from_slice(input);
        let buffer_meta = BufferMeta::add(buffer.make_mut(), &mut meta_buffer);

        // Act
        let _ = buffer_meta.remove();

        // Assert
        //assert_eq!(s, true);

        for b in buffer.iter_meta::<BufferMeta>() {
            // fail if we get here. There shouldn't be any BufferMetas
            assert_eq!(false, true, "A BufferMeta was found on buffer: {:#?}", b);
        }
    }
}
