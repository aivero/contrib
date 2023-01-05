from build import *


class Libxrender(Recipe):
    description = "X Rendering Extension client library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "pkgconf/[^1.7.3]",
        "xorg-util-macros/[^1.19.2]",
        "xtrans/[^1.4.0]",
    )
    requires = ("libx11/[^1.6.12]",)

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/libXrender-{self.version}.tar.gz"
        )
