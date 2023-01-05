from build import *


class Gtk4(GstRecipe):
    settings = GstRecipe.settings + ("compiler",)
    description = "GObject-based multi-platform GUI toolkit"
    license = "LGPL"
    options = {
        "shared": [True, False],
        "introspection": [True, False],
        "x11": [True, False],
    }
    default_options = (
        "shared=True",
        "introspection=True",
        "x11=True",
    )
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.61.1]",
        "gi-docgen/[^2021.8]",
        "shaderc/[>=2021.3]",
        "sassc/[^3.6.2]",
        "gobject-introspection/[^1.70.0]",
        # "docbook-xsl/[^1.79.2]",
        "wayland-protocols/[^1.25]",
        "python-docutils/[>=0.16]",
        "python-gobject/[^3.40.1]",
    )
    requires = (
        "gdk-pixbuf2/[^2.42.6]",
        "libepoxy/[^1.5.9]",
        "graphene/[^1.10.6]",
        # "iso-codes/[^4.9.0]",
        "wayland/[^1.20.0]",
        # "libcups/[^]",
    )

    def requirements(self):
        self.requires(f"gst-plugins-bad/[~{self.settings.gstreamer}]")
        if self.options.x11:
            self.requires("libxkbcommon/[^1.0.1]")
            self.requires("libxi/[^1.7.1]")
            self.requires("libxcursor/[^1.2.0]")
            self.requires("libxinerama/[^1.1.4]")

    def source(self):
        self.get(f"https://github.com/GNOME/gtk/archive/{self.version}.tar.gz")

    def build(self):
        opts = {
            "gtk_doc": False,
            "demos": False,
            "build-tests": False,
            "build-examples": False,
            "introspection": self.options.introspection,
        }

        self.meson(opts)
