from build import *


class Nasm(Recipe):
    description = "80x86 assembler designed for portability and modularity"
    license = "BSD"
    requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")

    def source(self):
        self.get(
            f"https://www.nasm.us/pub/nasm/releasebuilds/{self.version}/nasm-{self.version}.tar.xz"
        )
