from build import *


class Libxrandr(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "X11 RandR extension library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.1]",
    )
    requires = (
        "libxrender/[^0.9.10]",
        "libxext/[^1.3.4]",
    )

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/libXrandr-{self.version}.tar.gz"
        )
