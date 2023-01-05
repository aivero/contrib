from build import *


class Zig(Recipe):
    description = "a general-purpose programming language and toolchain for maintaining robust, optimal, and reusable software"
    license = "MIT"

    def build(self):
        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch)]
        self.get(
            f"https://ziglang.org/download/{self.version}/zig-linux-{arch}-{self.version}.tar.xz"
        )

    def package(self):
        self.copy("lib/*", src=self.src)
        self.copy("zig", src=self.src)
