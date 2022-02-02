from build import *


class Libsoup3(Recipe):
    description = "HTTP client/server library for GNOME"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "gobject-introspection/[^1.70.0]",
        "meson/[>=0.57.2]",
        "vala/[^0.55.1]"
    )
    requires = (
        "glib/[^2.70.3]",
    )

    def source(self):
        self.get(f"https://gitlab.gnome.org/GNOME/libsoup/-/archive/{self.version}/libsoup-{self.version}.tar.gz")
