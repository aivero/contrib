#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate glib_sys;
extern crate gst_sys;
extern crate gst_video_sys;

pub type BufferMeta = _BufferMeta;
pub type TagsMeta = _TagsMeta;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _BufferMeta {
    pub meta: gst_sys::GstMeta,
    pub buffer: *mut gst_sys::GstBuffer,
}

unsafe impl Sync for _BufferMeta {}
unsafe impl Send for _BufferMeta {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _TagsMeta {
    pub meta: gst_sys::GstMeta,
    pub tags: *mut gst_sys::GstTagList,
}

unsafe impl Sync for _TagsMeta {}
unsafe impl Send for _TagsMeta {}

extern "C" {
    pub fn buffer_meta_api_get_type() -> glib_sys::GType;
    pub fn buffer_meta_get_info() -> *const gst_sys::GstMetaInfo;
    pub fn buffer_meta_get(buffer: *mut gst_sys::GstBuffer) -> *mut BufferMeta;
    pub fn buffer_meta_add(
        buffer: *mut gst_sys::GstBuffer,
        buffer_meta: *mut gst_sys::GstBuffer,
    ) -> *mut BufferMeta;
    pub fn tags_meta_api_get_type() -> glib_sys::GType;
    pub fn tags_meta_get_info() -> *const gst_sys::GstMetaInfo;
    pub fn tags_meta_get(buffer: *mut gst_sys::GstBuffer) -> *mut TagsMeta;
    pub fn tags_meta_add(
        buffer: *mut gst_sys::GstBuffer,
        tags_meta: *mut gst_sys::GstTagList,
    ) -> *mut TagsMeta;
}
