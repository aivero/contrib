from build import *


class P11Kit(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Loads and enumerates PKCS#11 modules"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "pkgconf/[^1.7.3]",
        "libtasn1/[^4.16.0]",
        "libffi/[^3.3]",
    )

    def source(self):
        self.get(
            f"https://github.com/p11-glue/p11-kit/releases/download/{self.version}/p11-kit-{self.version}.tar.xz"
        )

    def build(self):
        args = [
            "--without-trust-paths",
        ]
        self.autotools(args)
