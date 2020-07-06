import os

from conans import *


class LibepoxyConan(ConanFile):
    name = "libepoxy"
    description = "Library handling OpenGL function pointer management"
    license = "MIT"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    build_requires = (
        "meson/[^0.51.2]",
    )
    requires = (
        "base/[^1.0.0]",
        "libx11/[^1.6.8]",
        "mesa/[^19.2.0]",
    )

    def source(self):
        tools.get(f"https://github.com/anholt/libepoxy/archive/{self.version}.tar.gz")

    def build(self):
        args = ["--auto-features=disabled", "-Dglx=yes", "-Dx11=true", "-Dtests=false"]
        meson = Meson(self)
        meson.configure(source_folder=f"{self.name, self.version), args=args}-{pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"}")
        meson.install()
