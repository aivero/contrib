from build import *


class SvtHevc(Recipe):
    description = "The Scalable Video Technology for HEVC Encoder"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.15.3]",
        "yasm/[^1.3.0]",
    )

    def source(self):
        self.get(f"https://github.com/OpenVisualCloud/SVT-HEVC/archive/v{self.version}.tar.gz")
