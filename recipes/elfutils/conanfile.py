from build import *


class Elfutils(Recipe):
    description = "Handle ELF object files and DWARF debugging information"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = "zlib/[^1.2.11]"

    def source(self):
        self.get(
            f"https://sourceware.org/elfutils/ftp/{self.version}/elfutils-{self.version}.tar.bz2"
        )

    def build(self):
        args = ["--disable-libdebuginfod", "--disable-debuginfod", "--disable-werror"]
        env = {"CFLAGS": "-Wno-error -Wno-null-dereference"}
        self.autotools(args, env=env)
