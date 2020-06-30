import os

from conans import *


class LibepoxyConan(ConanFile):
    description = "Library handling OpenGL function pointer management"
    license = "MIT"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    build_requires = (
        "generators/1.0.0",
        "meson/[^0.51.2]",
    )
    requires = (
        "libx11/[^1.6.8]",
        "mesa/[^19.2.0]",
    )

    def source(self):
        tools.get("https://github.com/anholt/libepoxy/archive/%s.tar.gz" % self.version)

    def build(self):
        args = ["--auto-features=disabled", "-Dglx=yes", "-Dx11=true", "-Dtests=false"]
        meson = Meson(self)
        meson.configure(source_folder="%s-%s" % (self.name, self.version), args=args, pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"))
        meson.install()
