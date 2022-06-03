from build import *


class Libxshmfence(Recipe):
    description = "Library that exposes a event API on top of Linux futexes"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.2]",
    )
    requires = ("xorgproto/[>=2020.1]",)

    def source(self):
        self.get(f"https://xorg.freedesktop.org/releases/individual/lib/libxshmfence-{self.version}.tar.gz")
