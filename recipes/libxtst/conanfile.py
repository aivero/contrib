from build import *


class LibxtstRecipe(Recipe):
    description = "X11 Testing Resource extension library"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.1]",
    )
    requires = (
        "libxext/[^1.3.4]",
        "libxfixes/[^5.0.3]",
        "libxi/[^1.7.1]",
    )

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/libXtst-{self.version}.tar.gz"
        )
