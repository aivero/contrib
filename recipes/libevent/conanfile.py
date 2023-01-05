from build import *


class Libevent(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Event notification library https://libevent.org"
    license = "BSD"
    exports = "uninstall.patch"
    build_requires = (
        "generators/1.0.0",
        "cmake/[>=3.15.3]",
    )
    requires = (
        "openssl1/[>=1.1.1h]",
        "zlib/[^1.2.11]",
    )

    def source(self):
        self.get(
            f"https://github.com/libevent/libevent/releases/download/release-{self.version}-stable/libevent-{self.version}-stable.tar.gz"
        )
        self.patch("uninstall.patch")
