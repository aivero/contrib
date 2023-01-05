from build import *


class Nettle(Recipe):
    description = "A low-level cryptographic library"
    license = "GPL"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")
    requires = ("gmp/[^6.2.0]",)

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/nettle/nettle-{self.version}.tar.gz")
