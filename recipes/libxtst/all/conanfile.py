from build import *


class LibxtstRecipe(Recipe):
    description = "X11 Testing Resource extension library"
    license = "custom"
    build_requires = (
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.1]",
    )
    requires = (
        "base/[^1.0.0]",
        "libx11/[^1.6.8]",
        "libxext/[^1.3.4]",
        "libxfixes/[^5.0.3]",
        "libxi/[^1.7.1]",
    )

    def source(self):
        self.get(f"https://xorg.freedesktop.org/releases/individual/lib/libXtst-{self.version}.tar.gz")

    def build(self):
        args = [
            "--disable-static",
        ]
        self.autotools(args)
