from build import *


class XorgProtoRecipe(Recipe):
    description = "combined X.Org X11 Protocol headers"
    license = "custom"
    build_requires = (
        "meson/[^0.55.3]",
        "xorg-util-macros/[^1.19.1]",
    )

    def source(self):
        self.get(f"https://xorg.freedesktop.org/archive/individual/proto/xorgproto-{self.version}.tar.bz2")
