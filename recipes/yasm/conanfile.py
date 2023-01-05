from build import *


class Yasm(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Yasm is a complete rewrite of the NASM assembler under the new BSD License"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )

    def source(self):
        self.get(f"http://www.tortall.net/projects/yasm/releases/yasm-{self.version}.tar.gz")
