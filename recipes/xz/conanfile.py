from build import *


class Xz(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Library and command line tools for XZ and LZMA compressed files"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://tukaani.org/xz/xz-{self.version}.tar.gz")
