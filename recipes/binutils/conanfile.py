from build import *


class Binutils(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A set of programs to assemble and manipulate binary and object files"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "git/[^2.30.0]",
    )
    requires = (
        "zlib/[^1.2.11]",
        "libelf/[^0.8.13]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/binutils/binutils-{self.version}.tar.xz")
