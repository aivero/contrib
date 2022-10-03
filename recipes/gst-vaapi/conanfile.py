from build import *


class GstVaapi(GstRecipe):
    description = "Hardware-accelerated video decoding, encoding and processing on Intel graphics through VA-API"
    license = "LGPL"
    options = {
        "drm": [True, False],
        "egl": [True, False],
        "encoders": [True, False],
        "glx": [True, False],
        "x11": [True, False],
        "wayland": [True, False],
        "tests": [True, False],
    }
    default_options = (
        "drm=True",
        "egl=True",
        "encoders=True",
        "glx=True",
        "x11=True",
        "wayland=False",
        "tests=True",
    )
    exports = ["vaapi_env.sh", "README_gst-vaapi.adoc"]
    build_requires = (
        "cc/[^1.0.0]",
        "gobject-introspection/[^1.59.3]",
        "meson/[>=0.62.0]",
    )
    requires = (
        "intel-media-driver/[^22.4.3]",
        "eudev/[^3.2.9]",
    )

    def requirements(self):
        self.requires(f"gst-plugins-bad/[~{self.settings.gstreamer}]")

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
        source_folder = os.path.join(self.src, "subprojects", "gstreamer-vaapi")
        opts = {}
        if "1.21" in self.version:
            opts = {
                "drm": self.options.drm,
                "egl": self.options.egl,
                "encoders": self.options.encoders,
                "glx": self.options.glx,
                "x11": self.options.x11,
                "wayland": self.options.wayland,
                "tests": self.options.tests,
            }
        else:
            opts = {
                "with_drm": self.options.drm,
                "with_egl": self.options.egl,
                "with_encoders": self.options.encoders,
                "with_glx": self.options.glx,
                "with_x11": self.options.x11,
                "with_wayland": self.options.wayland,
                "tests": self.options.tests,
            }

        self.meson(opts, source_folder)

    def package(self):
        super().package()
        self.copy("*.adoc")
        self.copy("*.sh", dst="scripts")
