from conans import *


class LibxcursorConan(ConanFile):
    description = "X cursor management library"
    license = "custom"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    build_requires = (
        "autotools/[^1.0.0]",
        "xorg-util-macros/[^1.19.1]",
    )
    requires = (
        "generators/[^1.0.0]",
        "libxrender/[^0.9.10]",
        "libxfixes/[^5.0.3]",
    )

    def source(self):
        tools.get(f"https://xorg.freedesktop.org/releases/individual/lib/libXcursor-{self.version}.tar.gz")

    def build(self):
        args = ["--disable-static"]
        autotools = AutoToolsBuildEnvironment(self)
        with tools.chdir(f"libXcursor-{self.version}"):
            autotools.configure(args=args)
            autotools.install()
