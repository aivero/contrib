from build import *


class Gmp(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A free library for arbitrary precision arithmetic"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "m4/[^1.4.18]",
    )

    def source(self):
        self.get(f"https://gmplib.org/download/gmp/gmp-{self.version}.tar.xz")
