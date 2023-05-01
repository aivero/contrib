from build import *


class GstLibav(GstRecipe):
    description = "GStreamer plugin for the libav* library (former FFmpeg)"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.62.0]",
    )
    requires = ("ffmpeg/[^4.1]",)

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/3044b0992f5cccbff7ad0e36302093f735c282ec.tar.gz"
        )

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-libav")
        self.meson({}, source_folder)
