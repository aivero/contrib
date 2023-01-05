from build import *


class Orc(Recipe):
    description = "Optimized Inner Loop Runtime Compiler"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )

    def source(self):
        self.get(f"https://github.com/GStreamer/orc/archive/{self.version}.tar.gz")

    def build(self):
        opts = {
            "tools": True,
        }
        self.meson(opts)
