from build import *


class GstDepthMeta(GstProject):
    license = "MIT"
    description = "Library to stream depth video"
    exports_sources = ("meson.build", "meson.build.conan", "src/*")
    build_requires = ("cc/[^1.0.0]", "meson/[>=0.57.2]")

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

    def source(self):
        self.run("mkdir gst-depth-meta")
        self.run("mv src gst-depth-meta/")
        self.run("mv meson.build.conan gst-depth-meta/meson.build")
        self.run("rm meson.build")

    def build(self):
        self.meson(source_folder="gst-depth-meta")

    def package(self):
        self.copy("*.h", dst="include", keep_path=False)
