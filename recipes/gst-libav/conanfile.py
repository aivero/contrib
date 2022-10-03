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
        if "1.21" in self.version:
            # until the changes from https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2132 and https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2432 are tagged we need to use a commit of the main branch
            self.get(
                f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/f8d8d67b8bc61fddd64ff648abd363d893a235a9.tar.gz"
            )
        else:
            self.get(
                f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/{self.version}.tar.gz"
            )

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-libav")
        self.meson({}, source_folder)
