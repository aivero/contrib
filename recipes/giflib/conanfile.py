from build import *


class Giflib(Recipe):
    description = "Library for reading and writing gif images"
    license = "custom"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")

    def source(self):
        self.get(f"https://downloads.sourceforge.net/project/giflib/giflib-{self.version}.tar.gz")

    def build(self):
        args = [
            f"PREFIX={self.package_folder}",
        ]
        self.make(args)
