from build import *


class Gtk3(Recipe):
    settings = Recipe.settings + ("compiler",)
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
        "meson/[>=0.55.3]",
        "gettext/[^0.21]",
        "wayland-protocols/[^1.25]",
    )
    requires = (
        "libepoxy/[^1.5.4]",
        "at-spi2-atk/[^2.38.0]",
        "gdk-pixbuf2/[^2.40.0]",
    )

    def build_requirements(self):
        if self.options.introspection:
            self.build_requires(
                "gobject-introspection/[^1.59.3]",
            )

    def requirements(self):
        if self.options.x11:
            self.requires("libxkbcommon/[^1.0.1]")
            self.requires("libxi/[^1.7.1]")

    def source(self):
        self.get(f"https://github.com/GNOME/gtk/archive/{self.version}.tar.gz")

    def build(self):
        opts = {
            "wayland_backend": True,
            "tracker3": False,
            "gtk_doc": False,
            "man": False,
        }

        # Meson cannot properly check options for unknown reasons
        self.meson(opts, opt_check=False)
