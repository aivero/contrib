# gst-depth-meta

This repository contains the C API for adding MetaData onto GStreamer Buffers. Metadata currently comes in two forms:

1. BufferMeta, which allows developers to add buffers as metadata onto other buffers.
2. TagsMeta, which allows developers to tag buffers, such that their content can be identified later downstream.

## Use

This repository acts as an ABI for the gst-depth-meta-rs repository, which contains Rust bindings. The reason for that is that Aivero's custom GStreamer elements are written primarily in Rust.

The repository is also used by the gstreamer-colorizer, which can colorize depth-video.
