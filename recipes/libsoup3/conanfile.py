from build import *


class Libsoup3(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "HTTP client/server library for GNOME"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "gobject-introspection/[^1.70.0]",
        "meson/[>=0.57.2]",
        "vala/[^0.55.1]",
    )
    requires = (
        "glib-networking/[^2.70.1]",
        "libnghttp2/[^1.46.0]",
        "libpsl/[>=0.21.1]",
        "sqlite/[^3.30.1]",
    )

    def source(self):
        self.get(
            f"https://gitlab.gnome.org/GNOME/libsoup/-/archive/{self.version}/libsoup-{self.version}.tar.gz"
        )

    def build(self):
        opts = {
            "introspection": True,
        }
        self.meson(opts)
