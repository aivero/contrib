import os

from conans import *


class NcursesConan(ConanFile):
    description = "System V Release 4.0 curses emulation library"
    license = "Zlib"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    build_requires = (
        "generators/1.0.0",
        "gcc/[^7.4.0]",
    )

    def source(self):
        tools.get("https://ftp.gnu.org/pub/gnu/ncurses/ncurses-%s.tar.gz" % self.version)

    def build(self):
        args = [
            "--enable-overwrite",
            "--with-shared",
            "--with-cxx-shared",
            "--with-cxx-binding",
            "--enable-pc-files",
            "--with-pkg-config-libdir=%s" % os.path.join(self.package_folder, "lib", "pkgconfig"),
        ]
        with tools.chdir("%s-%s" % (self.name, self.version)):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.make()
            autotools.install()

    def package_info(self):
        self.env_info.TERMINFO = os.path.join(self.package_folder, "share", "terminfo")
