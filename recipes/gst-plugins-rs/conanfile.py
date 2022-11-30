from build import *
from conans.errors import ConanInvalidConfiguration
from conan.tools.files import get


class GstRecipe(GstRustProject):
    description = "GStreamer plugins and elements written in the Rust programming language."
    license = "MIT"
    exports = "*.patch"
    build_requires = ("rust/[^1.0.0]",)
    requires = ("rust-libstd/[^1.0.0]",)
    has_workspace = True

    def requirements(self):
        self.requires(f"gst-plugins-bad/[~{self.settings.gstreamer}]")

    def source(self):
        get(
            self,
            f"https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs/-/archive/{self.version}/gst-plugins-rs-{self.version}.tar.gz",
        )
        os.system(f"mv gst-plugins-rs-{self.version} {self.source_folder}/gst-plugins-rs")
        os.system(f"rm -rf gst-plugins-rs-{self.version}")

    def build(self):
        self.cargo(
            [
                "-p gst-plugin-webrtc",
                "-p gst-plugin-rtp",
            ]
        )
