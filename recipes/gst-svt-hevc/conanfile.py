from build import *


class GstSvtHevcRecipe(GstRecipe):
    settings = GstRecipe.settings + ("compiler",)
    description = "The Scalable Video Technology for HEVC Encoder GStreamer plugin"
    license = "LGPL"
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.18.4]", "yasm/[^1.3.0]")
    requires = ("svt-hevc/[^1.4.3]",)

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

    def source(self):
        self.get(f"https://github.com/OpenVisualCloud/SVT-HEVC/archive/v{self.version}.tar.gz")
