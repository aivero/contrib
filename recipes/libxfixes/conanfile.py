from build import *


class Libxfixes(Recipe):
    description = "X11 miscellaneous 'fixes' extension library"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.2]",
    )
    requires = ("libx11/[^1.6.12]",)

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/libXfixes-{self.version}.tar.gz"
        )
