from build import *


class GstLibavRecipe(GstRecipe):
    description = "GStreamer plugin for the libav* library (former FFmpeg)"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]", 
        "meson/[>=0.55.3]",
    )
    requires = (
        "gst-plugins-base/[^1.18]",
        "ffmpeg/[^4.1]",
    )

    def source(self):
        self.get(f"https://github.com/GStreamer/gst-libav/archive/{self.version}.tar.gz")