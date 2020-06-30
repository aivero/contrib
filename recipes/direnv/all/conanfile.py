import os

from conans import *


class DirenvConan(ConanFile):
    description = "A shell extension that manages your environment"
    license = "MIT"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}

    def source(self):
        tools.get("https://github.com/direnv/direnv/archive/v{}.tar.gz".format(self.version))

    build_requires = ("go/1.13.8",)
    requires = ("generators/1.0.0",)

    def build(self):
        env = {"DESTDIR": self.package_folder}
        with tools.chdir("%s-%s" % (self.name, self.version)), tools.environment_append(env):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.make()
            autotools.install()
