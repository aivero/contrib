from build import *


class Eigen(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = (
        "Lightweight C++ template library for vector and matrix math, a.k.a. linear algebra."
    )
    license = "GPL3"
    requires = ("cmake/[^3.18.0]",)

    def source(self):
        self.get(
            f"https://gitlab.com/libeigen/eigen/-/archive/{self.version}/eigen-{self.version}.tar.gz"
        )
