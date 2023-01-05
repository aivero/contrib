from build import *


class Libxpm(Recipe):
    description = "X11 pixmap library"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.1]",
    )
    requires = ("libxext/[^1.3.4]",)

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/libXpm-{self.version}.tar.gz"
        )
