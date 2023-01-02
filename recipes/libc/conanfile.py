from build import *


class Libc(Recipe):
    description = "Virtual libc"
    license = "MIT"
    options = {}
    default_options = {}
    settings = "libc"

    def requirements(self):
        if self.settings.libc == "glibc":
            self.requires("glibc/[~2.31]")
        if self.settings.libc == "musl":
            self.requires("musl/[~1.2.1]")
        if self.settings.libc == "bionic":
            self.requires("bionic/[~28]")
