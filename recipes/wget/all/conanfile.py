from build import *


class WgetRecipe(Recipe):
    description = "Network utility to retrieve files from the Web"
    license = "GPL3"
    build_requires = (
        "autotools/[^1.0.0]",
        "nettle/[^3.6]",
        "libtasn1/[^4.16.0]",
    )
    requires = ("gnutls/[^3.6.15]",)

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/wget/wget-{self.version}.tar.gz")
