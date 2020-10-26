from build import *


class LibxineramaRecipe(Recipe):
    description = "X11 Xinerama extension library"
    license = "custom"
    build_requires = (
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.1]",
    )
    requires = ("libxext/[^1.3.4]",)

    def source(self):
        self.get(f"https://xorg.freedesktop.org/releases/individual/lib/libXinerama-{self.version}.tar.gz")
