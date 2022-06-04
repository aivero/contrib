from build import *


class LinuxTools(Recipe):
    description = "Linux tools"
    license = "GPL"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")

    def source(self):
        self.get(f"https://cdn.kernel.org/pub/linux/kernel/v{self.version.split('.')[0]}.x/linux-{self.version}.tar.xz")

    def build(self):
        args = [
            "WERROR=0"
        ]
        self.make(args, os.path.join(self.src, "tools", "perf"))