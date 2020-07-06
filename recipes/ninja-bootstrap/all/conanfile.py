import os

from conans import *


class NinjaBootstrapConan(ConanFile):
    description = "Small build system with a focus on speed"
    license = "Apache"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}

    def source(self):
        tools.get(f"https://github.com/ninja-build/ninja/archive/v{self.version}.tar.gz")

    def build(self):
        with tools.chdir(f"ninja-{self.version}"):
            self.run("python configure.py --bootstrap")

    def package(self):
        self.copy(os.path.join(f"ninja-{self.version}", "ninja"), "bin", keep_path=False)
