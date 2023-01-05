from build import *


class Itstool(Recipe):
    description = "XML to PO and back again"
    license = "GPL"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")
    requires = ("libxml2/[^2.9.10]",)

    def requirements(self):
        self.requires(f"python/[^3.8]")

    def source(self):
        self.get(f"https://github.com/itstool/itstool/archive/{self.version}.tar.gz")
