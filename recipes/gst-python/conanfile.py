from build import *


class GstPython(GstRecipe):
    settings = GstRecipe.settings + ("compiler",)
    description = "Gstreamer Python bindings"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.62.0]",
    )
    requires = ("python-gobject/[^3.33.1]",)

    def requirements(self):
        self.requires(f"gst/[~{self.settings.gstreamer}]")

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/{self.version}.tar.gz"
        )

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-python")
        self.meson({}, source_folder)
