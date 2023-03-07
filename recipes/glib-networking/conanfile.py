from build import *
from os import path

class GlibNetworking(Recipe):
    description = "Network extensions for GLib"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.61.1]",
    )
    requires = (
        "glib/[^2.70.3]",
        "openssl/[^3.0.7]",
    )

    def source(self):
        self.get(
            f"https://gitlab.gnome.org/GNOME/glib-networking/-/archive/{self.version}/glib-networking-{self.version}.tar.gz"
        )

    def build(self):
        opts = {
            "gnutls": False,
            "openssl": True,
        }
        self.meson(opts)

    def package_info(self):
        self.runenv_info.GIO_EXTRA_MODULES = path.join(self.package_folder, "lib/gio/modules")

