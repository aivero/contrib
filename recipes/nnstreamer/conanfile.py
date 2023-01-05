from os import symlink
from build import *


class GstNnstreamer(GstRecipe):
    description = (
        "Neural Network (NN) Streamer, Stream Processing Paradigm for Neural Network Apps/Devices."
    )
    license = "LGPL"
    build_requires = ("cc/[^1.0.0]", "meson/[>=0.57.2]", "flex/[^2.6.4]", "bison/[^3.7.6]")

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")
        self.requires(f"tensorflow2-lite/[^2.4.3]")

    def source(self):
        self.get(
            f"https://github.com/nnstreamer/nnstreamer/archive/refs/tags/v{self.version}.tar.gz"
        )

    def build(self):
        opts = {
            "werror": False,
            "tflite2-support": True,
        }
        self.meson(opts)

    def package(self):
        self.copy(pattern="nnstreamer.ini", dst="config")
