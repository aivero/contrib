from build import *


class PixmanRecipe(Recipe):
    description = "Image processing and manipulation library"
    license = "custom"
    build_requires = ("meson/[^0.55.3]",)

    def source(self):
        self.get(f"https://xorg.freedesktop.org/releases/individual/lib/pixman-{self.version}.tar.gz")
