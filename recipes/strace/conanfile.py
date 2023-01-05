from build import *


class Strace(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A diagnostic, debugging and instructional userspace tracer"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = ("perl/[^5.30.0]",)

    def source(self):
        self.get(
            f"https://github.com/strace/strace/releases/download/v{self.version}/strace-{self.version}.tar.xz"
        )

    def build(self):
        args = ["--enable-mpers=no", "--disable-gcc-Werror"]
        self.autotools(args)
