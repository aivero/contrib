from build import *


class Pcre(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A library that implements Perl 5-style regular expressions"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = (
        "readline/[^8.1]",
        "zlib/[^1.2.11]",
        "bzip2/[^1.0.8]",
    )

    def source(self):
        self.get(
            f"https://sourceforge.net/projects/pcre/files/pcre/{self.version}/pcre-{self.version}.tar.bz2"
        )

    def build(self):
        args = [
            "--enable-unicode-properties",
            "--enable-pcre16",
            "--enable-pcre32",
            "--enable-jit",
            "--enable-pcregrep-libz",
            "--enable-pcregrep-libbz2",
        ]
        self.autotools(args)
