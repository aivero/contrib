from build import *


class Musl(Recipe):
    description = "Lightweight implementation of C standard library"
    license = "MIT"
    requires = ("linux-headers/[^5.4.50]",)

    def source(self):
        self.get(f"https://www.musl-libc.org/releases/musl-{self.version}.tar.gz")
