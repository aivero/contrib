from build import *


class Xmlto(Recipe):
    description = "Convert xml to many other formats"
    license = "GPL"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")
    requires = ("libxslt/[^1.1.34]",)

    def source(self):
        self.get(f"https://releases.pagure.org/xmlto/xmlto-{self.version}.tar.bz2")
