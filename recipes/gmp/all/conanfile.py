from build import *


class GmpRecipe(Recipe):
    description = "A free library for arbitrary precision arithmetic"
    license = "GPL"
    build_requires = (
        "make/[^4.3]",
        "m4/[^1.4.18]",
    )

    def source(self):
        self.get(f"https://gmplib.org/download/gmp/gmp-{self.version}.tar.xz")

    def build(self):
        args = [
            "--disable-shared",
        ]
        self.autotools(args)
