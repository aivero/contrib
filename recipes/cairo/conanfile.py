from build import *


class Cairo(Recipe):
    description = "2D graphics library with support for multiple output devices"
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
        "autotools/[^1.0.0]",
        "gobject-introspection/[^1.66.1]",
    )
    requires = (
        "glib/[^2.70.3]",
        "pixman/[^0.40.0]",
        "fontconfig/[^2.13.92]",
    )

    def build_requirements(self):
        if self.options.introspection:
            self.build_requires("gobject-introspection/[^1.66.1]")

    def requirements(self):
        self.requires("libxrender/[^0.9.10]")
        self.requires("libxext/[^1.3.4]")

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/cairo/cairo/-/archive/{self.version}/cairo-{self.version}.tar.gz"
        )

    def build(self):
        args = [
            "--disable-gl",
            "--enable-gobject",
        ]
        os.environ["CFLAGS"] += " -lpthread"
        self.autotools(args)
