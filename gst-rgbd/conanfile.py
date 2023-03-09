from build import *


class Rgbd(GstRustProject):
    description = "GStreamer plugin for demuxing and muxing `video/rgbd` streams"
    license = "MIT"
    build_requires = ("rust/[^1.0.0]",)
    requires = (
        f"gst-depth-meta/{branch()}",
        "capnproto/[^0.8.0]",
    )