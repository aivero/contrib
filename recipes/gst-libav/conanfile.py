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
            f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/47c183cdfdd75fc0baf8218aae6621df1fe4e87b.tar.gz"
        )

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-libav")
        self.meson({}, source_folder)
