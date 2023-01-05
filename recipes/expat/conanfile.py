from build import *


class Expat(Recipe):
    description = "An XML parser library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )

    def source(self):
        self.get(
            f"https://github.com/libexpat/libexpat/releases/download/R_{self.version.replace('.', '_')}/expat-{self.version}.tar.bz2"
        )
