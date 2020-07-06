import os

from conans import *


class GdbConan(ConanFile):
    description = "The GNU Debugger"
    license = "GPL3"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    build_requires = (
        "cc/[^1.0.0]",
        "texinfo/[^6.6]",
    )
    requires = (
        "base/[^1.0.0]",
        "python/[^3.7.4]",
        "ncurses/[^6.1]",
        "readline/[^8.0]",
    )

    def source(self):
        tools.get(f"https://ftp.gnu.org/gnu/gdb/gdb-{self.version}.tar.gz")

    def build(self):
        args = ["--enable-tui=yes", "--with-system-readline"]
        with tools.chdir(f"{self.name}-{self.version}"):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.make()
            autotools.install()

    def package_info(self):
        self.env_info.PYTHONPATH.append(os.path.join(self.package_folder, "share", "gdb", "python"))
