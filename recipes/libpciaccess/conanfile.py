from build import *


class LibPciAccess(Recipe):
    description = "Generic PCI access library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "xorg-util-macros/[^1.19.1]",
    )

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/libpciaccess-{self.version}.tar.gz"
        )
