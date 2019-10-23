import os

from conans import AutoToolsBuildEnvironment, ConanFile, tools


class GdbConan(ConanFile):
    name = "gdb"
    version = tools.get_env("GIT_TAG", "8.3")
    url = "https://gitlab.com/aivero/public/conan/conan-bison"
    description = "The GNU Debugger"
    license = "GPL3"
    settings = "os", "arch", "compiler", "build_type"
    generators = "env"

    def build_requirements(self):
        self.build_requires("gcc/[>=7.4.0]@%s/stable" % self.user)
        self.build_requires("python/[>=3.7.4]@%s/stable" % self.user)

    def requirements(self):
        self.requires("env-generator/[>=1.0.0]@%s/stable" % self.user)

    def source(self):
        tools.get("https://ftp.gnu.org/gnu/gdb/gdb-%s.tar.gz" % self.version)

    def build(self):
        with tools.chdir("%s-%s" % (self.name, self.version)):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure()
            autotools.make()
            autotools.install()
