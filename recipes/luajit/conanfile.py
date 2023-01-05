from build import *


class Luajit(Recipe):
    description = "Just-in-time compiler and drop-in replacement for Lua 5.1"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )

    def source(self):
        self.get(f"https://github.com/openresty/luajit2/archive/refs/tags/v{self.version}.tar.gz")

    def build(self):
        args = [
            f"PREFIX={self.package_folder}",
            f"DEFAULT_CC={self.env['CC']}",
        ]
        self.make(args)
