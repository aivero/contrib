from build import *


class Libxxf86vm(Recipe):
    description = "X11 XFree86 video mode extension library"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.2]",
        "xorgproto/[>=2020.1]",
    )
    requires = ("libxext/[^1.3.4]",)

    def source(self):
        self.get(f"https://xorg.freedesktop.org/releases/individual/lib/libXxf86vm-{self.version}.tar.gz")
