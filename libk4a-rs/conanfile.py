from build import *

class Libk4aSys(GstRustProject):
    description = "Rust FFI bindings for Azure Kinect SDK"
    license = "LGPL"
    build_requires = (
        "rust/[^1.0.0]",
    )
    requires = (
        "libk4a/[^1.4.1]",
    )