from build import *


class Libtiff(CppRecipe):
    description = "Library for manipulation of TIFF images"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.0]",
        "mesa/[>=21.0.0]",
    )
    requires = (
        "zlib/[^1.2.11]",
        "libjpeg-turbo/[^2.0.4]",
    )

    def source(self):
        self.get(f"https://download.osgeo.org/libtiff/tiff-{self.version}.tar.gz")
