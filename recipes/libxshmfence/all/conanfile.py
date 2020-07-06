from conans import *


class LibxshmfenceConan(ConanFile):
    name = "libxshmfence"
    description = "Library that exposes a event API on top of Linux futexes"
    license = "custom"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    build_requires = (
        "pkgconf/[^1.6.3]",
        "xorg-util-macros/[^1.19.1]",
    )
    requires = (
        "base/[^1.0.0]",
        "xorgproto/[^2019.1]",
    )

    def source(self):
        tools.get(f"https://xorg.freedesktop.org/releases/individual/lib/libxshmfence-{self.version}.tar.gz")

    def build(self):
        args = ["--disable-static"]
        autotools = AutoToolsBuildEnvironment(self)
        with tools.chdir(f"{self.name}-{self.version}"):
            autotools.configure(args=args)
            autotools.install()
