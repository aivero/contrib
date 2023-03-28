from build import *


class LibBoost(Recipe):
    settings = GstRecipe.settings + ("compiler",)
    description = "Free C++ Source libraries"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://github.com/boostorg/boost/releases/download/boost-{self.version}/boost-{self.version}.tar.gz")
