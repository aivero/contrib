from build import *

class GstreamerFrameAligner(GstRustProject):
    license = "MIT"
    author = ("Joao Alves <joao.alves@aivero.com>", "Niclas Overby <niclas.overby@aivero.com>")
    description = "Gstreamer align frames utilities"
    build_requires = (
        "rust/[^1.0.0]",
        "cmake/[^3.18.4]",
        "capnproto/[^0.8.0]",
    )
    requires = (f"gst-depth-meta/{branch()}")