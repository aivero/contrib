from build import *


class AutoconfArchive(CppRecipe):
    description = "A collection of freely re-usable Autoconf macros"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(
            f"https://ftpmirror.gnu.org/autoconf-archive/autoconf-archive-{self.version}.tar.xz"
        )
