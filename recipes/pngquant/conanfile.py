from build import *


class Pngquant(Recipe):
    description = "Command-line utility to quantize PNGs down to 8-bit paletted PNGs"
    license = "BSD"
    build_requires = ("autotools/1.0.0",)
    requires = (
        "libpng/[^1.6.37]",
        "libimagequant/[^2.12.6]",
        "openmp/[^11.0.0]",
    )

    def source(self):
        self.get(
            f"https://github.com/kornelski/pngquant/archive/{self.version}/pngquant-{self.version}.tar.gz"
        )

    def build(self):
        args = [
            "--with-openmp",
        ]
        self.autotools(args)
