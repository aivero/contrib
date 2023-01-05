from build import *


class Mpc(Recipe):
    description = "Library for the arithmetic of complex numbers with arbitrarily high precision"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )
    requires = ("mpfr/[^4.1.0]",)

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/mpc/mpc-{self.version}.tar.gz")
