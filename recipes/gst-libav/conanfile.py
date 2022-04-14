from build import *


class GstLibav(GstRecipe):
    description = "GStreamer plugin for the libav* library (former FFmpeg)"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )
    requires = ("ffmpeg/[^4.1]",)

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

    def source(self):
        if "1.21" in self.version:
            # until https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2132 is merged (and tagged) we need to use slomos branch
            self.get(f"https://gitlab.freedesktop.org/slomo/gstreamer/-/archive/rfc6051.tar.gz")
        else:           
            self.get(f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/{self.version}.tar.gz")

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-libav")
        self.meson({}, source_folder)
