from build import *


class Libiconv(Recipe):
    description = "GNU charset conversion library"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/pub/gnu/libiconv/libiconv-{self.version}.tar.gz")