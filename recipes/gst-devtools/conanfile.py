from build import *


class GstDevtools(GstRecipe):
    description = "Development and debugging tools for GStreamer"
    license = "LGPL"
    options = {
        "gtk_doc": [True, False],
        "introspection": [True, False],
        "tests": [True, False],
        "nls": [True, False],
    }
    default_options = (
        "gtk_doc=False",
        "introspection=False",
        "tests=True",
        "nls=False",
    )
    build_requires = (
        "cc/[>=1.0.0]",
        "meson/[>=0.51.2]",
        "pkgconf/[^1.7.3]",
    )
    requires = ("json-glib/[>=1.6.2]",)

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

    def source(self):
        if "1.21" in self.version:
            # until the changes from https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2132 and https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2432 are tagged we need to use a commit of the main branch
            self.get(f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/3487c81ac28bd2c1b196dff748965543c8ebcf3d.tar.gz")
        else:           
            self.get(f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/{self.version}.tar.gz")

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-devtools")
        opts = {
            "debug_viewer": False,
            "doc": self.options.gtk_doc,
            "introspection": self.options.introspection,
            "nls": self.options.nls,
            "tests": self.options.tests,
            "validate": True,
        }
        self.meson(opts, source_folder)
