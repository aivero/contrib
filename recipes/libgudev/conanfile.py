from build import *


class GLib(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "libgudev, a library providing GObject bindings for libudev"
    license = "LGPL"
    build_requires = ("cc/[^1.0.0]", "meson/[>=0.55.3]")
    requires = (
        "glib/[^2.70.3]",
        "eudev/[^3.2.9]",
    )

    def source(self):
        version = self.version.replace(".", "")
        self.get(
            f"https://gitlab.gnome.org/GNOME/libgudev/-/archive/{version}/libgudev-{version}.tar.gz"
        )

    def build(self):
        opts = {}
        self.meson(opts)
