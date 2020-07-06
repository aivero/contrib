from conans import *


class LibimagequantConan(ConanFile):
    name = "libimagequant"
    description = "Library for high-quality conversion of RGBA images to 8-bit indexed-color (palette) images"
    license = "BSD"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    build_requires = ("autotools/1.0.0",)
    requires = ("base/[^1.0.0]",)

    def source(self):
        tools.get(f"https://github.com/ImageOptim/libimagequant/archive/{self.version}/libimagequant-{self.version}.tar.gz")

    def build(self):
        args = [
            "--with-openmp",
        ]
        env = {
            "DESTDIR": self.package_folder,
        }
        with tools.chdir(f"{self.name}-{self.version}"), tools.environment_append(env):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.install()
