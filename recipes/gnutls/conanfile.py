from build import *


class Gnutls(Recipe):
    description = "A library which provides a secure layer over a reliable transport layer"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "pkgconf/[^1.7.3]",
    )
    requires = (
        "p11-kit/[^0.23.21]",
        "nettle/[^3.6]",
        "libtasn1/[^4.16.0]",
        "zlib/[^1.2.11]",
    )

    def source(self):
        self.get(f"https://www.gnupg.org/ftp/gcrypt/gnutls/v3.7/gnutls-{self.version}.tar.xz")

    def build(self):
        args = [
            "--with-zlib",
            "--with-included-unistring",
            "--disable-tests",
        ]
        self.autotools(args)
