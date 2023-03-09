# gst-rgbd 

GStreamer plugin for demuxing and muxing video/rgbd streams using `rgbddemux` and `rgbdmux` respectively.

`rgbddemux` - GStreamer element for demuxing a single `video/rgbd` stream that creates a `src_%s` pad with `video/x-raw` caps for each detected stream.

`rgbdmux` - GStreamer element for muxing multiple `video/x-raw` on its `sink_%s` sink pads into a single `video/rgbd`.

# Getting started

> Note: This repo builds and installs **only** `rgbddemux` and `rgbdmux`. Please head to the `Aivero RGB-D Toolkit` to install a complete set of elements for handling RGB-D cameras.

## Install a tagged release

You may use conan to install a pre-built release of the gst-realsense package:

```bash
conan install gst-rgbd/0.1.0@ -if installation
export GST_PLUGIN_PATH=$GST_PLUGIN_PATH:$PWD/installation
# And validate that the plugins are properly installed
gst-inspect-1.0 rgbd
```
