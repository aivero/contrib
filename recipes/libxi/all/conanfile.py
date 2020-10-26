from build import *


class LibxiRecipe(Recipe):
    description = "X11 Input extension library"
    license = "custom"
    build_requires = (
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.1]",
    )
    requires = (
        "libxext/[^1.3.4]",
        "libxfixes/[^5.0.3]",
    )

    def source(self):
        self.get(f"https://xorg.freedesktop.org/releases/individual/lib/libXi-{self.version}.tar.gz")
