from build import *


class Libxcursor(Recipe):
    description = "X cursor management library"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.1]",
    )
    requires = (
        "libxrender/[^0.9.10]",
        "libxfixes/[^5.0.3]",
    )

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/libXcursor-{self.version}.tar.gz"
        )
