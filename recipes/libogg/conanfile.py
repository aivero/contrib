from build import *


class Libogg(Recipe):
    description = "Ogg bitstream and framing library"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )
    def source(self):
        self.get(f"https://github.com/xiph/ogg/archive/refs/tags/v{self.version}.zip")