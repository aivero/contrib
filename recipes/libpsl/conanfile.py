from build import *


class Libpsl(Recipe):
    description = "Public Suffix List library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "libxslt/[^1.1.34]",
    )

    def source(self):
        self.get(f"https://github.com/rockdaboot/libpsl/releases/download/{self.version}/libpsl-{self.version}.tar.gz")