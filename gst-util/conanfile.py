from build import *


class GstUtil(GstRustProject):
    description = "Utility library for making it a bit easier to work with gstreamer from rust"
    license = "MIT"
    build_requires = ("rust/[^1.0.0]")
    
    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")
