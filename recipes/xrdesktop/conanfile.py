from build import *


class Xrdesktop(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A library for XR interaction with classical desktop compositors"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "python-gobject/[^3.33.1]",
    )
    requires = ("gxr/[^0.15.2]",)

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/xrdesktop/xrdesktop/-/archive/{self.version}/xrdesktop-{self.version}.tar.gz"
        )
