from build import *


class Libpng(Recipe):
    description = "A collection of routines used to create PNG format graphics files"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/3.18.4",
    )
    requires = ("zlib/[^1.2.11]",)

    def source(self):
        self.get(
            f"https://downloads.sourceforge.net/sourceforge/libpng/libpng-{self.version}.tar.xz"
        )

    def build(self):
        defs = {
            "PNG_SHARED": self.options.shared,
            "PNG_STATIC": not self.options.shared,
        }
        self.cmake(defs)
