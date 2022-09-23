from build import *


class GstRtspServer(GstRecipe):
    description = "A framework for streaming media"
    license = "LGPL"
    exports = "*.patch"
    options = {
        "examples": [True, False],
        "tests": [True, False],
        "introspection": [True, False],
        "rtspclientsink": [True, False],
    }
    default_options = (
        "examples=False",
        "tests=False",
        "introspection=True",
        "rtspclientsink=True",
    )
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.62.0]",
        "bison/[^3.3]",
        "flex/[^2.6.4]",
        "gobject-introspection/[^1.59.3]",
    )

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

    def source(self):
        if "1.21" in self.version:
            # until the changes from https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2132 and https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2432 are tagged we need to use a commit of the main branch
            self.get(
                f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/11e4eb5490de11d3680d4ca875bbec6b0d751017.tar.gz"
            )
        else:
            self.get(
                f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/{self.version}.tar.gz"
            )

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-rtsp-server")
        opts = {
            "examples": self.options.examples,
            "tests": self.options.tests,
            "introspection": self.options.introspection,
            "rtspclientsink": self.options.rtspclientsink,
        }
        self.meson(opts, source_folder)
