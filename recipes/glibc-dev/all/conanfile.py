from conans import *


class GlibcDevConan(ConanFile):
    name = "glibc-dev"
    description = "glibc development files"
    license = "GPL"

    def source(self):
        tools.get(f"https://ftp.gnu.org/gnu/glibc/glibc-{self.version}.tar.xz")

    def build(self):
        autotools = AutoToolsBuildEnvironment(self)
        autotools.configure(configure_dir=f"glibc-{self.version}")
        autotools.make(target="install-headers")
