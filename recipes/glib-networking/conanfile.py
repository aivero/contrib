from build import *


class GlibNetworking(Recipe):
    description = "Network extensions for GLib"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.61.1]",
    )
    requires = (
        "glib/[^2.70.3]",
        "gnutls/[^3.6.15]",
    )

    def source(self):
        self.get(
            f"https://gitlab.gnome.org/GNOME/glib-networking/-/archive/{self.version}/glib-networking-{self.version}.tar.gz"
        )

    def build(self):
        opts = {"gnutls": True}
        self.meson(opts)
