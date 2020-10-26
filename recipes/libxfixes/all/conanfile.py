from build import *


class LibxfixesRecipe(Recipe):
    description = "X11 miscellaneous 'fixes' extension library"
    license = "custom"
    build_requires = (
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.2]",
    )
    requires = ("libx11/[^1.6.12]",)

    def source(self):
        self.get(f"https://xorg.freedesktop.org/releases/individual/lib/libXfixes-{self.version}.tar.gz")

    def build(self):
        args = [
            "--disable-static",
        ]
        self.autotools(args)
