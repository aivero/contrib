from build import *


class X265(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "x265 is the leading H.265 / HEVC encoder software library"
    license = "GPL"
    exports = "no-integrated-as.patch"
    options = {"high_bit_depth": [True, False], "main12": [True, False], "hdr10": [True, False]}
    default_options = "high_bit_depth=False", "main12=True", "hdr10=False"
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.18.4]", "nasm/[^2.15.05]", "binutils/[^2.36.1]")

    def source(self):
        self.get(
            f"https://bitbucket.org/multicoreware/x265_git/downloads/x265_{self.version}.tar.gz"
        )
        self.patch("no-integrated-as.patch")

    def build(self):
        defs = {
            "HIGH_BIT_DEPTH": self.options.high_bit_depth,
            "MAIN12": self.options.main12,
            "ENABLE_HDR10_PLUS": self.options.hdr10,
        }
        self.cmake(defs, source_folder=os.path.join(self.src, "source"))
