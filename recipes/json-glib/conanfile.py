from build import *


class JsonGlibBase(Recipe):
    description = "A well-groomed and well-maintained collection of GStreamer plugins and elements"
    license = "GPL"
    build_requires = (
        "cc/[>=1.0.0]",
        "meson/[>=0.51.2]",
        "gettext/[>=0.20.1]",
        "gobject-introspection/[>=1.59.3]",
    )
    requires = ("glib/[^2.70.3]",)

    def source(self):
        self.get(
            f"https://gitlab.gnome.org/GNOME/json-glib/-/archive/{self.version}/json-glib-{self.version}.tar.gz"
        )
