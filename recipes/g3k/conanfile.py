from build import *


class Gxr(Recipe):
    description = "A 3DUI widget toolkit"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )
    requires = (
        "gxr/[^0.16.0]",
        "shaderc/[^2021.3]",
        "libcanberra/[^0.30]",
    )

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/xrdesktop/g3k/-/archive/{self.version}/g3k-{self.version}.tar.gz"
        )
