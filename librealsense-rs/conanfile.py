from build import *


class LibrealsenseRs(GstRustProject):
    description = "Rust FFI bindings for librealsense"
    license = "MIT"
    build_requires = ("rust/[^1.0.0]",)
    requires = ("librealsense/[^2.39.0]",)
