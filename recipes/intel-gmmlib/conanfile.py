from build import *


class IntelGmmlib(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Intel Graphics Memory Management Library"
    license = "MIT"
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.18.4]")

    def source(self):
        self.get(f"https://github.com/intel/gmmlib/archive/intel-gmmlib-{self.version}.tar.gz")
