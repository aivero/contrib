from build import *


class Freetype(Recipe):
    description = "FreeType is a software library to render fonts"
    license = "FTL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def requirements(self):
        if self.name == "freetype":
            self.requires("harfbuzz/[^2.7.2]")
        else:
            self.requires("libpng/[^1.6.37]", "private")

    def source(self):
        self.get(
            f"https://download-mirror.savannah.gnu.org/releases/freetype/freetype-{self.version}.tar.xz"
        )

    def build(self):
        # Force use autotools
        self.autotools()
