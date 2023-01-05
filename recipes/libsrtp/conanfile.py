from build import *


class Libsrtp(Recipe):
    description = "Library for SRTP (Secure Realtime Transport Protocol)"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )
    requires = ("libssl/[^1.0.0]",)
    exports_sources = ("libsrtp2.pc",)

    def source(self):
        self.get(f"https://github.com/cisco/libsrtp/archive/v{self.version}.tar.gz")

    def build(self):
        defs = {
            "ENABLE_OPENSSL": True,
        }
        self.cmake(defs)

    def package(self):
        self.copy("libsrtp2.pc", dst="lib/pkgconfig")
        self.copy("*crypto_types.h", dst="include/srtp2", keep_path=False)
