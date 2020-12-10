from build import *


class XzRecipe(Recipe):
    description = "Library and command line tools for XZ and LZMA compressed files"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://tukaani.org/xz/xz-{self.version}.tar.gz")
