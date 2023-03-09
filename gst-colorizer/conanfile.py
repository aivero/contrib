from build import *


class GstColorizer(GstProject):
    license = "MIT"
    description = "Library to stream depth video"
    exports_sources = ("CMakeLists.txt", "src/*")
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.18.4]")

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")