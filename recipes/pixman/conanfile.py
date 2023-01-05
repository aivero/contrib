from build import *


class Pixman(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Image processing and manipulation library"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/pixman-{self.version}.tar.gz"
        )
