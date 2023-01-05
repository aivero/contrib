from build import *


class Libxcb(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "X11 client-side library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "pkgconf/[^1.7.3]",
    )
    requires = (
        ("xcb-proto/[^1.13]", "private"),
        ("libpthread-stubs/[^0.4]", "private"),
        "libxau/[^1.0.9]",
    )

    def source(self):
        self.get(f"https://xcb.freedesktop.org/dist/libxcb-{self.version}.tar.xz")
