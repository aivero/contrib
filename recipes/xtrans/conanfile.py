from build import *


class Xtrans(Recipe):
    description = "X transport library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/xtrans-{self.version}.tar.gz"
        )
