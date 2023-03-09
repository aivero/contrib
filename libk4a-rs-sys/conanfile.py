from build import *

class Libk4aSys(GstRustProject):
    description = "Rust FFI bindings for Azure Kinect SDK"
    license = "MIT"
    build_requires = (
        "rust/[^1.0.0]",
    )
    requires = (
        f"gst-depth-meta/{branch()}",
        "libk4a/[^1.4.1]",
    )