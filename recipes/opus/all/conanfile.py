from conans import *


class OpusConan(ConanFile):
    name = "opus"
    description = "Modern audio compression for the internet"
    license = "BSD"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}

    def source(self):
        tools.get(f"https://archive.mozilla.org/pub/opus/opus-{self.version}.tar.gz")

    def build(self):
        args = ["--disable-static"]
        with tools.chdir(f"{self.name}-{self.version}"):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.install()
