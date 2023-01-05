from build import *


class Openjpeg2(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "An open source JPEG 2000 codec, version 2.4.0"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.0]",
    )
    requires = (
        "libpng/[^1.6.37]",
        "libtiff/[^4.3.0]",
    )

    def source(self):
        self.get(f"https://github.com/uclouvain/openjpeg/archive/v{self.version}.tar.gz")
