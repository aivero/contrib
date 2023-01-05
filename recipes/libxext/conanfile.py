from build import *


class Libxext(Recipe):
    description = "X11 miscellaneous extensions library"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "pkgconf/[^1.7.3]",
        "xorg-util-macros/[^1.19.2]",
    )
    requires = ("libx11/[^1.6.12]",)

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/libXext-{self.version}.tar.gz"
        )
