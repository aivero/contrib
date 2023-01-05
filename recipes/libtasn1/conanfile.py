from build import *


class Libtasn1(Recipe):
    description = "The ASN.1 library used in GNUTLS"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/libtasn1/libtasn1-{self.version}.tar.gz")
